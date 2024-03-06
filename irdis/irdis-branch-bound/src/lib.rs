use std::num::NonZeroUsize;

use irdis_instance::{schedule::Schedule, Instance, Solve};

mod cost;

mod node;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchBound {
    pub horizon: Option<NonZeroUsize>,
}

impl BranchBound {
    pub fn new() -> Self {
        Self::with_rolling_horizon(None)
    }

    pub fn with_rolling_horizon<H>(horizon: H) -> Self
    where
        H: Into<Option<NonZeroUsize>>,
    {
        Self {
            horizon: horizon.into(),
        }
    }
}

impl Default for BranchBound {
    fn default() -> Self {
        Self::new()
    }
}

impl Solve for BranchBound {
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>> {
        node::branch_and_bound(instance, self.horizon)
    }
}

fn separation_identical_sets(instance: &Instance) -> Vec<Vec<usize>> {
    let mut sets = Vec::<Vec<_>>::with_capacity(instance.flights().len().min(1));

    'unclassified: for unclassified in 0..instance.flights().len() {
        'sets: for set in &mut sets {
            for classified in set.iter().copied() {
                let other = (0..instance.flights().len())
                    .filter(|other| ![unclassified, classified].contains(other));
                for other in other {
                    let sep_unclassified = instance.separations().get(unclassified, other);
                    let sep_unclassified_rev = instance.separations().get(other, unclassified);

                    let sep_classified = instance.separations().get(classified, other);
                    let sep_classified_rev = instance.separations().get(other, classified);

                    if sep_unclassified != sep_classified
                        || sep_unclassified_rev != sep_classified_rev
                    {
                        continue 'sets;
                    }
                }
            }

            set.push(unclassified);
            continue 'unclassified;
        }

        let set = vec![unclassified];
        sets.push(set);
    }

    sets
}
