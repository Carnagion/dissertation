//! Core types and definitions for runway sequencing and de-icing problems.

#![deny(rust_2018_idioms)]
#![warn(missing_docs)]

use std::time::Duration;

use serde::{Deserialize, Serialize};

use serde_with::{serde_as, DurationSeconds};

pub mod flight;
use flight::Flight;

pub mod schedule;
use schedule::Schedule;

pub mod sep;
use sep::{Separations, SeparationsAsSeconds, SeparationsMut};

pub mod solve;
use solve::Solve;

/// A runway sequencing problem instance.
#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Instance {
    flights: Box<[Flight]>,
    #[serde_as(as = "SeparationsAsSeconds")]
    separations: Separations,
    /// The maximum runway hold duration allowed for all aircraft in the instance.
    #[serde_as(as = "DurationSeconds")]
    pub max_runway_hold_duration: Duration,
}

impl Instance {
    /// Creates a new instance with the specified parameters, returning [`None`] if the number of
    /// aircraft does not match the number of rows or columns in the separation matrix.
    pub fn new<F>(
        flights: F,
        separations: Separations,
        max_runway_hold_duration: Duration,
    ) -> Option<Self>
    where
        F: Into<Box<[Flight]>>,
    {
        let flights = flights.into();
        (flights.len() == separations.len()).then_some(Self {
            flights,
            separations,
            max_runway_hold_duration,
        })
    }

    /// Extracts a slice of all aircraft in the instance.
    pub fn flights(&self) -> &[Flight] {
        &self.flights
    }

    /// Extracts a mutable slice of all aircraft in the instance.
    pub fn flights_mut(&mut self) -> &mut [Flight] {
        &mut self.flights
    }

    /// Extracts a boxed slice of aircraft from the instance, consuming it in the process.
    pub fn into_flights(self) -> Box<[Flight]> {
        self.flights
    }

    /// Returns the separation matrix of all aircraft in the instance.
    pub fn separations(&self) -> &Separations {
        &self.separations
    }

    /// Returns a mutable separation matrix of all aircraft in the instance.
    ///
    /// # Note
    ///
    /// A [`&mut Separations`] cannot be returned due to the logical invariant that must be upheld by [`Instance`].
    /// See the documentation for [`sep`] and [`SeparationsMut`] for further details.
    pub fn separations_mut(&mut self) -> SeparationsMut<'_> {
        SeparationsMut {
            inner: &mut self.separations,
        }
    }

    /// Extracts the separation matrix from the instance, consuming it in the process.
    pub fn into_separations(self) -> Separations {
        self.separations
    }

    /// Extracts a boxed slice of aircraft and separation matrix from the instance, consuming it in the process.
    pub fn into_flights_and_separations(self) -> (Box<[Flight]>, Separations) {
        (self.flights, self.separations)
    }

    /// Solves the instance using a default value of a [`Solve`]r.
    pub fn solve<S>(&self) -> Option<Vec<Schedule>>
    where
        S: Solve + Default,
    {
        let solver = S::default();
        solver.solve(self)
    }

    /// Solves the instance using the given [`Solve`]r.
    pub fn solve_with<S>(&self, solver: &S) -> Option<Vec<Schedule>>
    where
        S: Solve,
    {
        solver.solve(self)
    }
}
