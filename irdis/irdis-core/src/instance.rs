use std::str::FromStr;

use csv::{ReaderBuilder, Trim};

use serde::{Deserialize, Serialize};

use thiserror::Error;

use crate::solve::Solve;

pub mod op;

pub mod aircraft;

pub mod schedule;
use schedule::RunwaySchedule;

mod separation;
pub use separation::SeparationSets;

mod row;
pub use row::InstanceRow;

mod duration;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Instance {
    rows: Vec<InstanceRow>,
}

impl Instance {
    pub fn new(rows: Vec<InstanceRow>) -> Result<Self, InstanceError> {
        // Check that the number of rows equals the number of separations per row
        rows.iter()
            .all(|row| row.separations.len() == rows.len())
            .then_some(Self { rows })
            .ok_or(InstanceError::MismatchedLengths)
    }
}

#[derive(Clone, Debug, Error)]
pub enum InstanceError {
    #[error("number of rows does not match number of separations per row")]
    MismatchedLengths,
}

impl Instance {
    pub fn rows(&self) -> &[InstanceRow] {
        &self.rows
    }

    pub fn into_rows(self) -> Vec<InstanceRow> {
        self.rows
    }
}

impl Instance {
    pub fn solve<S>(&self) -> RunwaySchedule
    where
        S: Solve + Default,
    {
        self.solve_with(&S::default())
    }

    pub fn solve_with<S>(&self, solver: &S) -> RunwaySchedule
    where
        S: Solve,
    {
        solver.solve(self)
    }
}

impl FromStr for Instance {
    type Err = ParseInstanceError;

    fn from_str(csv: &str) -> Result<Self, Self::Err> {
        let rows = ReaderBuilder::new()
            .has_headers(false) // NOTE: This allows a variable number of fields, needed to parse separations
            .comment(Some(b'#'))
            .trim(Trim::All)
            .from_reader(csv.as_bytes())
            .into_deserialize()
            .collect::<Result<_, _>>()?;

        let instance = Self::new(rows)?;
        Ok(instance)
    }
}

#[derive(Debug, Error)]
pub enum ParseInstanceError {
    #[error(transparent)]
    Invalid(#[from] InstanceError),
    #[error("instance {}", .0)]
    Csv(#[from] csv::Error),
}
