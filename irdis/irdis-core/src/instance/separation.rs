use std::time::Duration;

use crate::instance::Instance;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SeparationSets(pub Vec<Vec<usize>>);

impl Instance {
    pub fn separation(&self, before_idx: usize, after_idx: usize) -> Option<Duration> {
        // NOTE: Each aircraft lists its separations assuming it is going
        //       after the one it is listed. Therefore, to get the separation
        //       between aircraft A and B, we need the separation entry for B
        //       in A's separation list.
        self.rows()
            .get(after_idx)?
            .separations
            .get(before_idx)
            .copied()
    }

    pub fn separation_sets(&self) -> SeparationSets {
        let mut sets = SeparationSets(Vec::with_capacity(1));

        // Go through every aircraft that is not in a separation set yet
        'unclassified: for unclassified in 0..self.rows.len() {
            // Check it against every aircraft in every separation set
            'sets: for set in &mut sets.0 {
                for classified in set.iter().copied() {
                    // Compare the two aircraft with every other aircraft except themselves
                    let aircraft = (0..self.rows.len())
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

                // If here, then the set is not skipped and it must be separation-identical
                // to all aircraft in the set, so we put it in the set and move on to the
                // next aircraft
                set.push(unclassified);
                continue 'unclassified;
            }

            // If here, then the aircraft did not match any separation set, so we
            // create a new set for it and put it there
            let set = vec![unclassified];
            sets.0.push(set);
        }

        sets
    }
}
