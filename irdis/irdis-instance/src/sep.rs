use std::{
    ops::{Deref, DerefMut, Index, IndexMut},
    time::Duration,
};

use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Vec<Vec<Duration>>", into = "Vec<Vec<Duration>>")]
pub struct Separations {
    // NOTE: This is a two-dimensional matrix of size `len * len` represented
    //       as a one-dimensional slice.
    data: Box<[Duration]>,
    len: usize,
}

// NOTE: This wrapper is necessary since the `try_from`/`into` container attributes
//       supported by `serde` do not take into account the `DurationMinutes` wrapper
//       intended for use with `serde_with`.
serde_with::serde_conv! {
    pub(super) SeparationsAsMinutes,
    Separations,
    |separations: &Separations| -> Vec<Vec<Duration>> {
        let mut grid = separations.to_grid();
        grid.iter_mut().flatten().for_each(|dur| *dur /= 60);
        grid
    },
    |grid: Vec<Vec<Duration>>| -> Result<Separations, SeparationsLenError> {
        let mut separations = Separations::try_from(grid)?;
        separations.data.iter_mut().for_each(|dur| *dur *= 60);
        Ok(separations)
    }
}

impl Separations {
    pub fn new<S>(data: S, len: usize) -> Option<Self>
    where
        S: Into<Box<[Duration]>>,
    {
        let data = data.into();
        (data.len() == len * len).then_some(Self { data, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, from: usize, to: usize) -> Option<&Duration> {
        let idx = self.index_of(from, to);
        self.data.get(idx)
    }

    pub fn get_mut(&mut self, from: usize, to: usize) -> Option<&mut Duration> {
        let idx = self.index_of(from, to);
        self.data.get_mut(idx)
    }

    pub fn index_of(&self, from: usize, to: usize) -> usize {
        from * self.len + to
    }

    pub fn into_boxed_slice(self) -> Box<[Duration]> {
        self.data
    }

    pub fn to_grid(&self) -> Vec<Vec<Duration>> {
        let len = self.len;
        let mut grid = Vec::with_capacity(len);
        grid.extend((0..len).map(|idx| self.data[idx * len..(idx + 1) * len].to_vec()));
        grid
    }
}

impl Deref for Separations {
    type Target = [Duration];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Separations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl Index<(usize, usize)> for Separations {
    type Output = Duration;

    fn index(&self, (from, to): (usize, usize)) -> &Self::Output {
        let idx = self.index_of(from, to);
        self.data.index(idx)
    }
}

impl IndexMut<(usize, usize)> for Separations {
    fn index_mut(&mut self, (from, to): (usize, usize)) -> &mut Self::Output {
        let idx = self.index_of(from, to);
        self.data.index_mut(idx)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Error)]
#[error("separation matrix has invalid size")]
pub struct SeparationsLenError;

impl From<Separations> for Vec<Vec<Duration>> {
    fn from(separations: Separations) -> Self {
        let mut data = Vec::from(separations.data);
        let mut grid = Self::with_capacity(separations.len);
        for _ in 0..separations.len {
            grid.push(data.drain(..separations.len).collect());
        }
        grid
    }
}

impl TryFrom<Vec<Vec<Duration>>> for Separations {
    type Error = SeparationsLenError;

    fn try_from(separations: Vec<Vec<Duration>>) -> Result<Self, Self::Error> {
        separations
            .iter()
            .all(|row| row.len() == separations.len())
            .then(|| {
                let len = separations.len();
                let data = separations.into_iter().flatten().collect();
                Self { data, len }
            })
            .ok_or(SeparationsLenError)
    }
}

pub struct SeparationsMut<'a> {
    pub(super) inner: &'a mut Separations,
}

impl SeparationsMut<'_> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get(&self, from: usize, to: usize) -> Option<&Duration> {
        self.inner.get(from, to)
    }

    pub fn get_mut(&mut self, from: usize, to: usize) -> Option<&mut Duration> {
        self.inner.get_mut(from, to)
    }
}

impl Deref for SeparationsMut<'_> {
    type Target = [Duration];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

// NOTE: `SeparationsMut<'_>` does not impl `DerefMut<Target = Separations>` since that would allow
//       obtaining a `&mut Separations`, which can then be easily swapped out using `mem::swap` and
//       friends, thereby breaking a containing `Instance`'s logical invariant.
impl DerefMut for SeparationsMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl Index<(usize, usize)> for SeparationsMut<'_> {
    type Output = Duration;

    fn index(&self, (from, to): (usize, usize)) -> &Self::Output {
        self.inner.index((from, to))
    }
}

impl IndexMut<(usize, usize)> for SeparationsMut<'_> {
    fn index_mut(&mut self, (from, to): (usize, usize)) -> &mut Self::Output {
        self.inner.index_mut((from, to))
    }
}
