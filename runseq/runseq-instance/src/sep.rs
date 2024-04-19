//! A two-dimensional, non-resizable aircraft separation matrix.
//!
//! Separation matrices are always square and cannot be resized without reassigning their value -
//! although their elements can be mutated.
//! This is due to the logical invariant that must be upheld by [`Instance`](crate::Instance) - the dimensions of
//! the separation matrix must match the number of aircraft in the instance.

use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut, Index, IndexMut},
    time::Duration,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use serde_with::{
    formats::{Format, Strict, Strictness},
    DeserializeAs,
    DurationSeconds,
    SerializeAs,
};

use thiserror::Error;

/// A square aircraft separation matrix represented as a one-dimensional slice.
///
/// The individual separation values in the matrix can be mutated, but the matrix as a whole cannot
/// be resized.
/// See the [module-level documentation](self) for more details.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(try_from = "Vec<Vec<Duration>>", into = "Vec<Vec<Duration>>")]
pub struct Separations {
    // NOTE: This is a two-dimensional matrix of size `len * len` represented
    //       as a one-dimensional slice.
    data: Box<[Duration]>,
    len: usize,
}

impl Separations {
    /// Creates a new separation matrix from the given separations and length, returning [`None`] if there
    /// are not exactly `len * len` elements.
    pub fn new<S>(data: S, len: usize) -> Option<Self>
    where
        S: Into<Box<[Duration]>>,
    {
        let data = data.into();
        (data.len() == len * len).then_some(Self { data, len })
    }

    /// Returns the length of the separation matrix.
    ///
    /// This is the number of elements per row or per column.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the separation matrix is empty, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a reference to the separation between two aircraft `from` and `to`, where `from` lands
    /// or takes off before `to`.
    pub fn get(&self, from: usize, to: usize) -> Option<&Duration> {
        let idx = self.index_of(from, to);
        self.data.get(idx)
    }

    /// Returns a mutable reference to the separation between two aircraft `from` and `to`, where `from` lands
    /// or takes off before `to`.
    pub fn get_mut(&mut self, from: usize, to: usize) -> Option<&mut Duration> {
        let idx = self.index_of(from, to);
        self.data.get_mut(idx)
    }

    /// Returns the one-dimensional index of a separation indexed by a row and column (i.e. preceeding and succeeding aircraft).
    pub fn index_of(&self, from: usize, to: usize) -> usize {
        from * self.len + to
    }

    /// Extracts the slice of separations from the matrix, consuming it in the process.
    pub fn into_boxed_slice(self) -> Box<[Duration]> {
        self.data
    }

    /// Converts the separation matrix into a two-dimensional grid of [`Vec`]s.
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

/// The error produced when trying to construct a [`Separations`] with invalid dimensions.
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

/// A wrapper type for a mutably borrowed separation matrix.
///
/// This type exists because an [`Instance`] cannot hand out a mutable reference to its [`Separations`] without
/// potentially breaking its logical invariant.
/// [`SeparationsMut`] bridges this gap, allows an instance's separation matrix to be mutated without being resized.
/// See the [module-level documentation](self) for more details.
pub struct SeparationsMut<'a> {
    pub(super) inner: &'a mut Separations,
}

impl SeparationsMut<'_> {
    /// Returns the length of the underlying separation matrix.
    ///
    /// See [`Separations::len`].
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Checks if the underlying separation matrix is empty.
    ///
    /// See [`Separations::is_empty`].
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Returns a reference to an element in the separation matrix.
    ///
    /// See [`Separations::get`].
    pub fn get(&self, from: usize, to: usize) -> Option<&Duration> {
        self.inner.get(from, to)
    }

    /// Returns a mutable reference to an element in the separation matrix.
    ///
    /// See [`Separations::get_mut`].
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

/// A helper type for use with [`serde_with`] to serialize separations as seconds.
pub struct SeparationsAsSeconds<Fmt = u64, Strt = Strict>(PhantomData<(Fmt, Strt)>);

impl<'de, Fmt, Strt> DeserializeAs<'de, Separations> for SeparationsAsSeconds<Fmt, Strt>
where
    Fmt: Format,
    Strt: Strictness,
    DurationSeconds<Fmt, Strt>: DeserializeAs<'de, Duration>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<Separations, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<Vec<DurationSeconds<Fmt, Strt>>>::deserialize_as(deserializer).and_then(|grid| {
            let separations = Separations::try_from(grid).map_err(serde::de::Error::custom)?;
            Ok(separations)
        })
    }
}

impl<Fmt, Strt> SerializeAs<Separations> for SeparationsAsSeconds<Fmt, Strt>
where
    Fmt: Format,
    Strt: Strictness,
    DurationSeconds<Fmt, Strt>: SerializeAs<Duration>,
{
    fn serialize_as<S>(separations: &Separations, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let grid = separations.to_grid();
        Vec::<Vec<DurationSeconds<Fmt, Strt>>>::serialize_as(&grid, serializer)
    }
}
