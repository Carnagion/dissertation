use std::time::Duration;

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use crate::time::{Ctot, DurationMinutes, TimeWindow};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum Flight {
    #[serde(rename = "arrival")]
    Arr(Arrival),
    #[serde(rename = "departure")]
    Dep(Departure),
}

impl Flight {
    pub fn base_time(&self) -> NaiveTime {
        match self {
            Self::Arr(arr) => arr.base_time,
            Self::Dep(dep) => dep.base_time,
        }
    }

    pub fn time_window(&self) -> &TimeWindow {
        match self {
            Self::Arr(arr) => &arr.window,
            Self::Dep(dep) => &dep.window,
        }
    }

    pub fn release_time(&self) -> NaiveTime {
        match self {
            Self::Arr(arr) => arr.release_time(),
            Self::Dep(dep) => dep.release_time(),
        }
    }

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
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Arrival {
    pub base_time: NaiveTime,
    pub window: TimeWindow,
}

impl Arrival {
    pub fn release_time(&self) -> NaiveTime {
        self.base_time.max(self.window.earliest)
    }
}

impl From<Arrival> for Flight {
    fn from(arr: Arrival) -> Self {
        Self::Arr(arr)
    }
}

#[serde_as] // NOTE: This must remain before the derive.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Departure {
    pub base_time: NaiveTime,
    pub window: TimeWindow,
    pub ctot: Ctot,
    #[serde_as(as = "DurationMinutes")]
    pub pushback_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub taxi_deice_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub deice_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub taxi_out_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub lineup_dur: Duration,
}

impl Departure {
    pub fn release_time(&self) -> NaiveTime {
        self.base_time
            .max(self.window.earliest)
            .max(self.ctot.earliest())
    }
}

impl From<Departure> for Flight {
    fn from(dep: Departure) -> Self {
        Self::Dep(dep)
    }
}
