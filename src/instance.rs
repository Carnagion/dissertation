use std::{str::FromStr, time::Duration};

use csv::{ReaderBuilder, Trim};

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use thiserror::Error;

pub mod aircraft;
use aircraft::Aircraft;

pub mod op;
use op::DepartureConstraints;

mod duration;
use duration::DurationMinutes;

#[serde_as] // NOTE: This must remain before the derive
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InstanceRow {
    pub aircraft: Aircraft,
    pub constraints: DepartureConstraints,
    #[serde_as(as = "Vec<DurationMinutes>")]
    pub separations: Vec<Duration>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(transparent)]
pub struct Instance(pub Vec<InstanceRow>);

impl Instance {
    pub fn rows(&self) -> &[InstanceRow] {
        &self.0
    }

    pub fn separation(&self, earlier_idx: usize, later_idx: usize) -> Option<Duration> {
        self.0.get(later_idx)?.separations.get(earlier_idx).copied()
    }

    pub fn separation_sets(&self) -> Vec<Vec<usize>> {
        let mut sets = Vec::<Vec<_>>::with_capacity(1);

        // Go through every aircraft that is not in a separation set yet
        'unclassified: for unclassified in 0..self.rows().len() {
            // Check it against every aircraft in every separation set
            'sets: for set in &mut sets {
                for classified in set.iter().copied() {
                    // Compare the two aircraft with every other aircraft except themselves
                    let aircraft = (0..self.rows().len())
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

#[derive(Debug, Error)]
pub enum ParseInstanceError {
    #[error("number of rows does not match number of separations per row")]
    UnequalSeparationLengths,
    #[error("{}", .0)]
    Csv(#[from] csv::Error),
}

impl FromStr for Instance {
    type Err = ParseInstanceError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let instance = ReaderBuilder::new()
            .has_headers(false) // NOTE: Needed so that the separations can be parsed from the end
            .comment(Some(b'#'))
            .trim(Trim::All)
            .from_reader(string.as_bytes())
            .into_deserialize()
            .collect::<Result<Self, _>>()?;

        // Check that the number of separations in each row equals the number of rows
        instance
            .0
            .iter()
            .all(|row| row.separations.len() == instance.0.len())
            .then_some(instance)
            .ok_or(ParseInstanceError::UnequalSeparationLengths)
    }
}

impl FromIterator<InstanceRow> for Instance {
    fn from_iter<I>(rows: I) -> Self
    where
        I: IntoIterator<Item = InstanceRow>,
    {
        Self(Vec::from_iter(rows))
    }
}
