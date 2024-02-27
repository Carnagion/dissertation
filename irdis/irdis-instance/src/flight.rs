use std::time::Duration;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use crate::{time::TimeWindow, DurationMinutes};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Flight {
    #[serde(rename = "arrival")]
    Arr(Arrival),
    #[serde(rename = "departure")]
    Dep(Departure),
}

impl Flight {
    pub fn time_window(&self) -> &TimeWindow {
        match self {
            Self::Arr(arr) => &arr.window,
            Self::Dep(dep) => &dep.ctot,
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

#[serde_as] // NOTE: This must remain before the derive
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Arrival {
    pub window: TimeWindow,
    #[serde_as(as = "DurationMinutes")]
    pub taxi_in_dur: Duration,
}

impl From<Arrival> for Flight {
    fn from(arr: Arrival) -> Self {
        Self::Arr(arr)
    }
}

#[serde_as] // NOTE: This must remain before the derive
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Departure {
    pub ctot: TimeWindow,
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

impl From<Departure> for Flight {
    fn from(dep: Departure) -> Self {
        Self::Dep(dep)
    }
}
