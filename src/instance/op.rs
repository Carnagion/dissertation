use std::str::FromStr;

use thiserror::Error;

use time::Time;

use crate::instance::SeparationId;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OpConstraint {
    pub kind: OpKind,
    pub earliest_time: Time,
    pub separation_id: SeparationId,
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid operation kind")]
pub struct ParseOpKindError;

impl FromStr for OpKind {
    type Err = ParseOpKindError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "D" => Ok(Self::Departure),
            "A" => Ok(Self::Arrival),
            _ => Err(ParseOpKindError),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AssignedOp {
    pub aircraft_idx: usize,
    pub kind: OpKind,
    pub earliest_time: Time,
    pub time: Time,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum OpKind {
    Arrival,
    Departure,
}
