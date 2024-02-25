use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::time::TimeWindow;

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
            _ => None,
        }
    }

    pub fn into_arrival(self) -> Option<Arrival> {
        match self {
            Self::Arr(arr) => Some(arr),
            _ => None,
        }
    }

    pub fn as_departure(&self) -> Option<&Departure> {
        match self {
            Self::Dep(dep) => Some(dep),
            _ => None,
        }
    }

    pub fn into_departure(self) -> Option<Departure> {
        match self {
            Self::Dep(dep) => Some(dep),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Arrival {
    pub window: TimeWindow,
    pub taxi_in_dur: Duration,
}

impl From<Arrival> for Flight {
    fn from(arr: Arrival) -> Self {
        Self::Arr(arr)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Departure {
    pub ctot: TimeWindow,
    pub pushback_dur: Duration,
    pub taxi_deice_dur: Duration,
    pub deice_dur: Duration,
    pub taxi_out_dur: Duration,
    pub lineup_dur: Duration,
}

impl From<Departure> for Flight {
    fn from(dep: Departure) -> Self {
        Self::Dep(dep)
    }
}
