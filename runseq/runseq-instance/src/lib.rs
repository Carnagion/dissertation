#![deny(rust_2018_idioms)]

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

#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Instance {
    flights: Box<[Flight]>,
    #[serde_as(as = "SeparationsAsSeconds")]
    separations: Separations,
    #[serde_as(as = "DurationSeconds")]
    pub max_runway_hold_duration: Duration,
}

impl Instance {
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

    pub fn flights(&self) -> &[Flight] {
        &self.flights
    }

    pub fn flights_mut(&mut self) -> &mut [Flight] {
        &mut self.flights
    }

    pub fn into_flights(self) -> Box<[Flight]> {
        self.flights
    }

    pub fn separations(&self) -> &Separations {
        &self.separations
    }

    pub fn separations_mut(&mut self) -> SeparationsMut<'_> {
        SeparationsMut {
            inner: &mut self.separations,
        }
    }

    pub fn into_separations(self) -> Separations {
        self.separations
    }

    pub fn into_flights_and_separations(self) -> (Box<[Flight]>, Separations) {
        (self.flights, self.separations)
    }

    pub fn solve<S>(&self) -> Option<Vec<Schedule>>
    where
        S: Solve + Default,
    {
        let solver = S::default();
        solver.solve(self)
    }

    pub fn solve_with<S>(&self, solver: &S) -> Option<Vec<Schedule>>
    where
        S: Solve,
    {
        solver.solve(self)
    }
}
