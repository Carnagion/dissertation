use std::time::Duration;

pub mod aircraft;
pub use aircraft::{Aircraft, Model, Registration, SizeClass};

mod op;
pub use op::{Op, OpKind};

mod separation;
pub use separation::{SeparationId, SeparationMatrix, SeparationRow};

pub struct Instance {
    pub aircraft: Vec<Aircraft>,
    pub operations: Vec<Op>,
    pub separations: SeparationMatrix,
}

impl Instance {
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
