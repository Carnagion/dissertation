use std::{num::ParseIntError, str::FromStr, time::Duration};

use thiserror::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeparationMatrix {
    pub rows: Vec<SeparationRow>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeparationRow {
    pub id: SeparationId,
    pub separations: Vec<Duration>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SeparationId(u8);

impl SeparationId {
    pub fn new(id: u8) -> Self {
        Self(id)
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseSeparationError {
    #[error("expected separation id")]
    ExpectedId,
    #[error("{}", .0)]
    InvalidNum(#[from] ParseIntError),
}

impl FromStr for SeparationMatrix {
    type Err = ParseSeparationError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let rows = string
            .lines()
            .map(SeparationRow::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { rows })
    }
}

impl FromStr for SeparationRow {
    type Err = ParseSeparationError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut nums = string.split_ascii_whitespace();

        let id = nums.next().ok_or(ParseSeparationError::ExpectedId)?;
        let id = id.parse()?;

        let separations = nums
            .map(|num| {
                let num = num.parse::<u64>()?;
                Ok::<_, ParseIntError>(Duration::from_secs(num * 60))
            })
            .collect::<Result<Vec<Duration>, _>>()?;

        Ok(Self { id, separations })
    }
}

impl FromStr for SeparationId {
    type Err = ParseIntError;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        id.parse().map(Self)
    }
}
