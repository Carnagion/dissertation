use std::{ops::RangeInclusive, time::Duration};

use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

use serde_with::{serde_as, DurationSeconds};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Flight {
    #[serde(rename = "arrival")]
    Arr(Arrival),
    #[serde(rename = "departure")]
    Dep(Departure),
}

impl Flight {
    pub fn as_arrival(&self) -> Option<&Arrival> {
        match self {
            Self::Arr(arr) => Some(arr),
            Self::Dep(_) => None,
        }
    }

    pub fn into_arrival(self) -> Option<Arrival> {
        match self {
            Self::Arr(arr) => Some(arr),
            Self::Dep(_) => None,
        }
    }

    pub fn as_departure(&self) -> Option<&Departure> {
        match self {
            Self::Dep(dep) => Some(dep),
            Self::Arr(_) => None,
        }
    }

    pub fn into_departure(self) -> Option<Departure> {
        match self {
            Self::Dep(dep) => Some(dep),
            Self::Arr(_) => None,
        }
    }

    pub fn earliest_time(&self) -> NaiveDateTime {
        match self {
            Self::Arr(arr) => arr.earliest_time,
            Self::Dep(dep) => dep.earliest_time,
        }
    }

    pub fn base_time(&self) -> NaiveDateTime {
        match self {
            Self::Arr(arr) => arr.base_time,
            Self::Dep(dep) => dep.base_time,
        }
    }

    pub fn window(&self) -> Option<&TimeWindow> {
        match self {
            Self::Arr(arr) => arr.window.as_ref(),
            Self::Dep(dep) => dep.window.as_ref(),
        }
    }

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Arrival {
    pub earliest_time: NaiveDateTime,
    pub base_time: NaiveDateTime,
    pub window: Option<TimeWindow>,
}

impl Arrival {
    pub fn release_time(&self) -> NaiveDateTime {
        match &self.window {
            Some(window) => self.earliest_time.max(self.base_time).max(window.earliest),
            None => self.base_time,
        }
    }
}

#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Departure {
    pub earliest_time: NaiveDateTime,
    pub base_time: NaiveDateTime,
    pub tobt: NaiveDateTime,
    #[serde_as(as = "DurationSeconds")]
    pub pushback_duration: Duration,
    pub deice: Option<Deice>,
    #[serde_as(as = "DurationSeconds")]
    pub taxi_duration: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub lineup_duration: Duration,
    pub ctot: Option<Ctot>,
    pub window: Option<TimeWindow>,
}

impl Departure {
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

#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Deice {
    #[serde_as(as = "DurationSeconds")]
    pub taxi_duration: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub duration: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub hot: Duration,
}

#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ctot {
    pub target: NaiveDateTime,
    #[serde_as(as = "DurationSeconds")]
    pub allow_early: Duration,
    #[serde_as(as = "DurationSeconds")]
    pub allow_late: Duration,
}

impl Ctot {
    pub fn earliest(&self) -> NaiveDateTime {
        self.target - self.allow_early
    }

    pub fn latest(&self) -> NaiveDateTime {
        self.target + self.allow_late
    }

    pub fn as_range(&self) -> RangeInclusive<NaiveDateTime> {
        self.earliest()..=self.latest()
    }
}

#[serde_as] // NOTE: This must remain before the derives for `Serialize` and `Deserialize`.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeWindow {
    pub earliest: NaiveDateTime,
    #[serde_as(as = "DurationSeconds")]
    pub duration: Duration,
}

impl TimeWindow {
    pub fn latest(&self) -> NaiveDateTime {
        self.earliest + self.duration
    }

    pub fn as_range(&self) -> RangeInclusive<NaiveDateTime> {
        self.earliest..=self.latest()
    }
}
