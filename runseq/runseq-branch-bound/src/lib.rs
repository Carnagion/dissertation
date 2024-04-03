#![deny(rust_2018_idioms)]

use std::num::NonZeroUsize;

use runseq_instance::{schedule::Schedule, solve::Solve, Instance};

mod complete_orders;

mod cost;
pub use cost::solution_cost;

mod search;

mod decomposed;

mod integrated;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct BranchBound {
    pub horizon: Option<NonZeroUsize>,
    pub deice_strategy: DeiceStrategy,
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum DeiceStrategy {
    ByTobt,
    ByCtot,
    #[default]
    Integrated,
}

impl Solve for BranchBound {
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>> {
        match self.deice_strategy {
            DeiceStrategy::ByTobt => {
                decomposed::branch_bound_rolling(instance, self.horizon, |dep, other| {
                    dep.tobt.cmp(&other.tobt)
                })
            },
            DeiceStrategy::ByCtot => {
                decomposed::branch_bound_rolling(instance, self.horizon, |dep, other| {
                    match dep.ctot.as_ref().zip(other.ctot.as_ref()) {
                        Some((dep_ctot, other_ctot)) => {
                            dep_ctot.earliest().cmp(&other_ctot.earliest())
                        },
                        None => dep.tobt.cmp(&other.tobt),
                    }
                })
            },
            DeiceStrategy::Integrated => integrated::branch_bound_rolling(instance, self.horizon),
        }
    }
}
