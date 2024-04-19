//! The [`Solve`] trait for representing runway sequencing solvers.

use crate::{schedule::Schedule, Instance};

/// A solver capable of producing solutions for a runway sequencing and de-icing [`Instance`].
pub trait Solve {
    /// Solves an [`Instance`] to produce a sequence of landing or take-off times and de-icing times.
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>>;
}
