//! Scheduled arrivals and departures.

use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

/// An arrival or departure that has been scheduled to land or take off.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Schedule {
    /// A scheduled arrival.
    #[serde(rename = "arrival")]
    Arr(ArrivalSchedule),
    /// A scheduled departure.
    #[serde(rename = "departure")]
    Dep(DepartureSchedule),
}

impl Schedule {
    /// Attempts to extract the scheduled aircraft as an arrival, returning [`None`] if it was a departure instead.
    pub fn as_arrival(&self) -> Option<&ArrivalSchedule> {
        match self {
            Self::Arr(sched) => Some(sched),
            Self::Dep(_) => None,
        }
    }

    /// Attempts to convert the scheduled aircraft into an arrival, returning [`None`] if it was a departure instead.
    pub fn into_arrival(self) -> Option<ArrivalSchedule> {
        match self {
            Self::Arr(sched) => Some(sched),
            Self::Dep(_) => None,
        }
    }

    /// Attempts to extract the scheduled aircraft into a departure, returning [`None`] if it was an arrival instead.
    pub fn as_departure(&self) -> Option<&DepartureSchedule> {
        match self {
            Self::Dep(sched) => Some(sched),
            Self::Arr(_) => None,
        }
    }

    /// Attempts to convert the scheduled aircraft into a departure, returning [`None`] if it was an arrival instead.
    pub fn into_departure(self) -> Option<DepartureSchedule> {
        match self {
            Self::Dep(sched) => Some(sched),
            Self::Arr(_) => None,
        }
    }

    /// Returns the index of the scheduled aircraft in its [`Instance`](crate::Instance).
    pub fn flight_index(&self) -> usize {
        match self {
            Self::Arr(arr) => arr.flight_index,
            Self::Dep(dep) => dep.flight_index,
        }
    }

    /// Returns the scheduled landing or take-off time of the aircraft.
    pub fn flight_time(&self) -> NaiveDateTime {
        match self {
            Self::Arr(arr) => arr.landing,
            Self::Dep(dep) => dep.takeoff,
        }
    }
}

impl From<ArrivalSchedule> for Schedule {
    fn from(sched: ArrivalSchedule) -> Self {
        Self::Arr(sched)
    }
}

impl From<DepartureSchedule> for Schedule {
    fn from(sched: DepartureSchedule) -> Self {
        Self::Dep(sched)
    }
}

/// A scheduled arrival.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ArrivalSchedule {
    /// The arrival's index in its [`Instance`](crate::Instance).
    pub flight_index: usize,
    /// The arrival's scheduled landing time.
    pub landing: NaiveDateTime,
}

/// A scheduled departure.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DepartureSchedule {
    /// The departure's index in its [`Instance`](crate::Instance).
    pub flight_index: usize,
    /// The departure's scheduled de-icing time, if any.
    pub deice: Option<NaiveDateTime>,
    /// The departure's scheduled take-off time.
    pub takeoff: NaiveDateTime,
}
