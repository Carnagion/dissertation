use std::time::Duration;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

pub mod flight;
use flight::Flight;

pub mod sep;
use sep::{Separations, SeparationsMut};

pub mod schedule;
use schedule::Schedule;

pub mod time;

use time::{DurationMinutes, SeparationsAsMinutes};

#[cfg(any(feature = "furini", feature = "xlsx"))]
pub mod convert;

#[serde_as] // NOTE: This must remain before the derive.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Instance {
    flights: Box<[Flight]>,
    #[serde_as(as = "SeparationsAsMinutes")]
    separations: Separations,
    #[serde_as(as = "DurationMinutes")]
    pub max_holdover_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub max_slack_dur: Duration,
}

impl Instance {
    pub fn new<F>(
        flights: F,
        separations: Separations,
        max_holdover_dur: Duration,
        max_slack_dur: Duration,
    ) -> Option<Self>
    where
        F: Into<Box<[Flight]>>,
    {
        let flights = flights.into();
        (flights.len() == separations.len()).then_some(Self {
            flights,
            separations,
            max_holdover_dur,
            max_slack_dur,
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
}

pub trait Solve {
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>>;
}
