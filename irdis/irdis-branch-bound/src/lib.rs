use std::num::NonZeroUsize;

use irdis_instance::{schedule::Schedule, Instance, Solve};

mod cost;
use cost::solution_cost;

mod complete_orders;

pub mod mode;
use mode::{integrated, DeiceMode};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchBound {
    pub deice_mode: DeiceMode,
    pub horizon: Option<NonZeroUsize>,
}

impl BranchBound {
    pub fn new() -> Self {
        Self {
            deice_mode: DeiceMode::default(),
            horizon: None,
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
        let solution = match self.deice_mode {
            DeiceMode::Sequential => todo!(),
            DeiceMode::Integrated => integrated::branch_and_bound(instance, self.horizon),
        }?;
        // TODO: Remove once testing is done
        println!("cost = {}", solution_cost(&solution, instance));
        Some(solution)
    }
}
