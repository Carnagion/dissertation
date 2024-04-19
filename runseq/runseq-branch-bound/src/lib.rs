//! A branch-and-bound algorithm for integrated as well as decomposed runway sequencing and de-icing.

#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

use std::num::NonZeroUsize;

use runseq_instance::{schedule::Schedule, solve::Solve, Instance};

mod complete_orders;

mod cost;
pub use cost::solution_cost;

mod search;

mod decomposed;

mod integrated;

/// A branch-and-bound solver for solving [`Instance`]s.
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct BranchBound {
    /// The size of the rolling horizon to use when solving an instance, if any.
    pub horizon: Option<NonZeroUsize>,
    /// The de-icing strategy to use when solving an instance.
    pub deice_strategy: DeiceStrategy,
}

/// Different de-icing strategies used for solving an [`Instance`].
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum DeiceStrategy {
    /// Decomposed de-icing by Target Off-Block Time (TOBT).
    ///
    /// Under this strategy, a de-icing queue is first generated after sorting departures by their TOBT.
    /// All aircraft are then assigned a landing or take-off time.
    ByTobt,
    /// Decomposed de-icing by Calculated Take-Off Time (CTOT).
    ///
    /// Under this strategy, a de-icing queue is first generated after sorting departures by their CTOT, or
    /// by their release time if they do not have a CTOT.
    /// All aircraft are then assigned a landing or take-off time.
    ByCtot,
    /// Integrated de-icing.
    ///
    /// This strategy assigns landings, take-offs, and de-icing times to all aircraft together, rather than
    /// solving one problem first.
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
