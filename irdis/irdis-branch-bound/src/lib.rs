use std::num::NonZeroUsize;

use irdis_instance::{schedule::Schedule, Instance, Solve};

mod cost;
use cost::solution_cost;

mod complete_orders;

mod search;
use search::branch_and_bound;

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
        let solution = branch_and_bound(instance, self.horizon)?;
        // TODO: Remove once testing is done
        println!("cost = {}", solution_cost(&solution, instance));
        Some(solution)
    }
}
