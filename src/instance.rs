use std::{num::ParseIntError, time::Duration};

use thiserror::Error;

use time::{error::Parse as ParseTimeError, macros::format_description, Time};

pub mod aircraft;
pub use aircraft::{Aircraft, Model, ParseClassError, Registration, SizeClass};

mod op;
pub use op::{AssignedOp, OpConstraint, OpKind, ParseOpKindError};

mod separation;
pub use separation::{ParseSeparationError, SeparationId, SeparationMatrix, SeparationRow};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Instance {
    pub aircraft: Vec<Aircraft>,
    pub op_constraints: Vec<OpConstraint>,
    pub separations: SeparationMatrix,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseInstanceError {
    #[error("expected operation count")]
    ExpectedCount,
    #[error("invalid operation count: {}", .0)]
    InvalidCount(ParseIntError),
    #[error("expected aircraft id")]
    ExpectedId,
    #[error("expected aircraft model")]
    ExpectedModel,
    #[error("expected aircraft size class")]
    ExpectedClass,
    #[error("{}", .0)]
    InvalidClass(#[from] ParseClassError),
    #[error("expected operation kind")]
    ExpectedOpKind,
    #[error("{}", .0)]
    InvalidOpKind(#[from] ParseOpKindError),
    #[error("expected operation time")]
    ExpectedTime,
    #[error("{}", .0)]
    InvalidTime(#[from] ParseTimeError),
    #[error("expected separation id")]
    ExpectedSeparationId,
    #[error("invalid separation id: {}", .0)]
    InvalidSeparationId(ParseIntError),
    #[error("{}", .0)]
    InvalidSeparationRow(#[from] ParseSeparationError),
}

impl Instance {
    pub fn parse(op_constraints: &str, separations: &str) -> Result<Self, ParseInstanceError> {
        let mut op_lines = op_constraints.lines();
        let separation_lines = separations.lines();

        let num_ops = op_lines
            .next()
            .ok_or(ParseInstanceError::ExpectedCount)?
            .parse()
            .map_err(ParseInstanceError::InvalidCount)?;

        let mut aircraft = Vec::with_capacity(num_ops);
        let mut op_constraints = Vec::with_capacity(num_ops);
        let mut sep_rows = Vec::with_capacity(num_ops);

        for (op_constraint, sep_row) in op_lines.zip(separation_lines) {
            let mut op_parts = op_constraint.split_ascii_whitespace();

            // Parse the aircraft registration
            let registration = op_parts.next().ok_or(ParseInstanceError::ExpectedId)?;
            let registration = Registration::new(registration.to_owned());

            // Parse the aircraft model
            let model = op_parts.next().ok_or(ParseInstanceError::ExpectedModel)?;
            let model = Model::new(model.to_owned());

            // Parse the aircraft size class
            let class = op_parts
                .next()
                .ok_or(ParseInstanceError::ExpectedClass)?
                .parse()?;

            // Add the aircraft to the list of known aircraft
            aircraft.push(Aircraft {
                registration,
                model,
                class,
            });

            // Parse the operation kind
            let kind = op_parts
                .next()
                .ok_or(ParseInstanceError::ExpectedOpKind)?
                .parse()?;

            // Parse the earliest time of the operation
            let earliest_time = op_parts.next().ok_or(ParseInstanceError::ExpectedTime)?;
            let earliest_time = Time::parse(earliest_time, format_description!("[hour][minute]"))?;

            // Parse the separation ID
            let separation_id = op_parts
                .next()
                .ok_or(ParseInstanceError::ExpectedSeparationId)?
                .parse()
                .map(SeparationId::new)
                .map_err(ParseInstanceError::InvalidSeparationId)?;

            // Add the operation to the aircraft operation list
            op_constraints.push(OpConstraint {
                kind,
                earliest_time,
                separation_id,
            });

            // Parse the separation row for that aircraft and add it to the separation matrix
            let sep_row = sep_row.parse()?;
            sep_rows.push(sep_row);
        }

        Ok(Self {
            aircraft,
            op_constraints,
            separations: SeparationMatrix { rows: sep_rows },
        })
    }

    pub fn separation(&self, from: usize, to: usize) -> Option<Duration> {
        self.separations
            .rows
            .get(from)?
            .separations
            .get(to)
            .copied()
    }

    pub fn separation_sets(&self) -> Vec<Vec<usize>> {
        let mut sets = Vec::<Vec<usize>>::with_capacity(1);

        // Go through every aircraft that is not in a separation set yet
        'unclassified: for unclassified in 0..self.aircraft.len() {
            // Check it against every aircraft in every separation set
            'sets: for set in &mut sets {
                for classified in set.iter().copied() {
                    // Compare the two aircraft with every other aircraft except themselves
                    let aircraft = (0..self.aircraft.len())
                        .filter(|&relative| relative != unclassified && relative != classified);
                    for relative in aircraft {
                        // Calculate their separations relative to the other aircraft
                        let sep_unclassified = self.separation(unclassified, relative);
                        let sep_unclassified_rev = self.separation(relative, unclassified);
                        let sep_classified = self.separation(classified, relative);
                        let sep_classified_rev = self.separation(relative, classified);

                        // Skip to next set if they are not separation-identical
                        if sep_classified != sep_unclassified
                            || sep_classified_rev != sep_unclassified_rev
                        {
                            continue 'sets;
                        }
                    }
                }

                // If here then the set is not skipped and it must be separation-identical to all aircraft in the set
                // Put it in the set and move on to the next aircraft
                set.push(unclassified);
                continue 'unclassified;
            }

            // If here then the aircraft did not match any separation set
            // Create a new set for it and put it there
            let set = vec![unclassified];
            sets.push(set);
        }

        sets
    }
}
