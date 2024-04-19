//! Arrivals and departures.

use std::{ops::RangeInclusive, time::Duration};

use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

use serde_with::{serde_as, DurationSeconds};

/// An aircraft that may be either an arrival or a departure.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Flight {
    /// An arrival.
    #[serde(rename = "arrival")]
    Arr(Arrival),
    /// A departure.
    #[serde(rename = "departure")]
    Dep(Departure),
}

impl Flight {
    /// Attempts to extract the aircraft as an arrival, returning [`None`] if it was a departure instead.
    pub fn as_arrival(&self) -> Option<&Arrival> {
        match self {
            Self::Arr(arr) => Some(arr),
            Self::Dep(_) => None,
        }
    }

    /// Attempts to convert the aircraft into an arrival, returning [`None`] if it was a departure instead.
    pub fn into_arrival(self) -> Option<Arrival> {
        match self {
            Self::Arr(arr) => Some(arr),
            Self::Dep(_) => None,
        }
    }

    /// Attempts to extract the aircraft into a departure, returning [`None`] if it was an arrival instead.
    pub fn as_departure(&self) -> Option<&Departure> {
        match self {
            Self::Dep(dep) => Some(dep),
            Self::Arr(_) => None,
        }
    }

    /// Attempts to convert the aircraft into a departure, returning [`None`] if it was an arrival instead.
    pub fn into_departure(self) -> Option<Departure> {
        match self {
            Self::Dep(dep) => Some(dep),
            Self::Arr(_) => None,
        }
    }

    /// Returns the earliest time the aircraft can be scheduled to land or take off.
    pub fn earliest_time(&self) -> NaiveDateTime {
        match self {
            Self::Arr(arr) => arr.earliest_time,
            Self::Dep(dep) => dep.earliest_time,
        }
    }

    /// Returns the base time of the aircraft, used for delay calculations.
    pub fn base_time(&self) -> NaiveDateTime {
        match self {
            Self::Arr(arr) => arr.base_time,
            Self::Dep(dep) => dep.base_time,
        }
    }

    /// Returns the hard time window of the aircraft, if any.
    pub fn window(&self) -> Option<&TimeWindow> {
        match self {
            Self::Arr(arr) => arr.window.as_ref(),
            Self::Dep(dep) => dep.window.as_ref(),
        }
    }

    /// Returns the release time of the aircraft.
    pub fn release_time(&self) -> NaiveDateTime {
        match self {
            Self::Arr(arr) => arr.release_time(),
            Self::Dep(dep) => dep.release_time(),
        }
    }
}

impl From<Arrival> for Flight {
    fn from(arr: Arrival) -> Self {
        Self::Arr(arr)
    }
}

impl From<Departure> for Flight {
    fn from(dep: Departure) -> Self {
        Self::Dep(dep)
    }
}

/// An arrival.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Arrival {
    /// The earliest time the arrival can be scheduled to land.
    pub earliest_time: NaiveDateTime,
    /// The base time of the arrival, used for delay calculations.
    pub base_time: NaiveDateTime,
    /// The hard time window of the arrival.
    pub window: Option<TimeWindow>,
}

impl Arrival {
    /// Returns the release time of the arrival.
    pub fn release_time(&self) -> NaiveDateTime {
        match &self.window {
            Some(window) => self.earliest_time.max(self.base_time).max(window.earliest),
            None => self.base_time,
        }
    }
}

/// A departure.
#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Departure {
    /// The earliest time the departure can be scheduled to take off.
    pub earliest_time: NaiveDateTime,
    /// The base time of the departure, used for delay calculations.
    pub base_time: NaiveDateTime,
    /// The Target Off-Block Time (TOBT) of the departure.
    pub tobt: NaiveDateTime,
    /// The amount of time taken by the aircraft to pushback.
    #[serde_as(as = "DurationSeconds")]
    pub pushback_duration: Duration,
    /// The de-icing parameters of the departure.
    pub deice: Option<Deice>,
    /// The amount of time taken by the aircraft to taxi out (after de-icing, if any).
    #[serde_as(as = "DurationSeconds")]
    pub taxi_duration: Duration,
    /// The amount of time taken by the aircraft to lineup on the runway.
    #[serde_as(as = "DurationSeconds")]
    pub lineup_duration: Duration,
    /// The Calculated Take-Off Time (CTOT) of the departure.
    pub ctot: Option<Ctot>,
    /// The hard time window of the departure.
    pub window: Option<TimeWindow>,
}

impl Departure {
    /// Returns the release time of the departure.
    pub fn release_time(&self) -> NaiveDateTime {
        let mut release = self.earliest_time.max(self.base_time);
        if let Some(window) = &self.window {
            release = release.max(window.earliest);
        }
        if let Some(ctot) = &self.ctot {
            release = release.max(ctot.earliest());
        }
        release
    }
}

/// A departure's de-icing parameters and information.
#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Deice {
    /// The amount of time taken to taxi from the runway to the de-icing station.
    #[serde_as(as = "DurationSeconds")]
    pub taxi_duration: Duration,
    /// The amount of time taken to apply the de-icing fluid.
    #[serde_as(as = "DurationSeconds")]
    pub duration: Duration,
    /// The departure's Holdover Time (HOT).
    #[serde_as(as = "DurationSeconds")]
    pub hot: Duration,
}

/// The Calculated Take-Off Time (CTOT) slot of a departure.
#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ctot {
    /// The target time in the CTOT slot.
    pub target: NaiveDateTime,
    /// How much time before the CTOT slot the departure is allowed to take off.
    #[serde_as(as = "DurationSeconds")]
    pub allow_early: Duration,
    /// How much time after the CTOT slot the departure is allowed to take off.
    #[serde_as(as = "DurationSeconds")]
    pub allow_late: Duration,
}

impl Ctot {
    /// Returns the earliest time of the CTOT slot.
    pub fn earliest(&self) -> NaiveDateTime {
        self.target - self.allow_early
    }

    /// Returns the latest time of the CTOT slot.
    pub fn latest(&self) -> NaiveDateTime {
        self.target + self.allow_late
    }

    /// Extracts the CTOT slot as an inclusive range from its earliest time to its latest time.
    pub fn as_range(&self) -> RangeInclusive<NaiveDateTime> {
        self.earliest()..=self.latest()
    }
}

/// A hard time window of an aircraft.
#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeWindow {
    /// The earliest time in the time window.
    pub earliest: NaiveDateTime,
    /// The duration of the time window.
    #[serde_as(as = "DurationSeconds")]
    pub duration: Duration,
}

impl TimeWindow {
    /// Returns the latest time in the time window.
    pub fn latest(&self) -> NaiveDateTime {
        self.earliest + self.duration
    }

    /// Extracts the time window as an inclusive range from its earliest time to its latest time.
    pub fn as_range(&self) -> RangeInclusive<NaiveDateTime> {
        self.earliest..=self.latest()
    }
}
