use std::num::NonZeroUsize;

use explore::explore_sep_sets;
use irdis_instance::{schedule::Schedule, Instance, Solve};

mod cost;
use cost::Cost;

mod explore;

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
        let mut sep_sets = separation_identical_sets(instance);
        let mut next_in_sep_sets = vec![0; sep_sets.len()];

        let mut current_solution = Vec::with_capacity(instance.flights().len());
        let mut best_solution = current_solution.clone();

        let mut horizon = match self.horizon {
            None => 0..instance.flights().len(),
            Some(horizon) => 0..usize::from(horizon).min(instance.flights().len()),
        };

        explore_sep_sets(
            instance,
            &sep_sets,
            &mut next_in_sep_sets,
            &mut current_solution,
            &mut best_solution,
            &mut Cost::default(),
            horizon.clone(),
        );

        while horizon.end < instance.flights().len() {
            next_in_sep_sets.fill(0);

            let sched = best_solution.drain(..).next()?;

            for sep_set in &mut sep_sets {
                sep_set.retain(|&flight_idx| flight_idx != sched.flight_index());
            }

            current_solution.push(sched);

            horizon.start += 1;
            horizon.end += 1;

            explore_sep_sets(
                instance,
                &sep_sets,
                &mut next_in_sep_sets,
                &mut current_solution,
                &mut best_solution,
                &mut Cost::default(),
                horizon.clone(),
            );
        }

        current_solution.extend(best_solution);
        Some(current_solution)
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
