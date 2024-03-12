use std::time::Duration;

#[cfg(feature = "serde")]
use cfg_eval::cfg_eval;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_with::serde_as;

pub mod flight;
use flight::Flight;

pub mod sep;
use sep::{Separations, SeparationsMut};

pub mod schedule;
use schedule::Schedule;

pub mod time;

#[cfg(feature = "serde")]
use time::{DurationMinutes, SeparationsAsMinutes};

#[cfg(any(feature = "furini", feature = "xlsx"))]
pub mod convert;

// NOTE: This must remain before the derive. The `cfg_eval` is to make the inner `cfg_attr` attributes
//       evaluate before `serde_as` is applied, which allows `serde_as` to function properly.
#[cfg_attr(feature = "serde", cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Instance {
    flights: Box<[Flight]>,
    #[cfg_attr(feature = "serde", serde_as(as = "SeparationsAsMinutes"))]
    separations: Separations,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
    pub max_holdover_dur: Duration,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
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
