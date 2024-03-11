use std::time::Duration;

#[cfg(feature = "serde")]
use cfg_eval::cfg_eval;

use chrono::NaiveTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_with::serde_as;

use crate::time::{Ctot, TimeWindow};

#[cfg(feature = "serde")]
use crate::time::DurationMinutes;

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Flight {
    #[cfg_attr(feature = "serde", serde(rename = "arrival"))]
    Arr(Arrival),
    #[cfg_attr(feature = "serde", serde(rename = "departure"))]
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

// NOTE: This must remain before the derive. The `cfg_eval` is to make the inner `cfg_attr` attributes
//       evaluate before `serde_as` is applied, which allows `serde_as` to function properly.
#[cfg_attr(feature = "serde", cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Departure {
    pub base_time: NaiveTime,
    pub window: TimeWindow,
    pub ctot: Ctot,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
    pub pushback_dur: Duration,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
    pub taxi_deice_dur: Duration,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
    pub deice_dur: Duration,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
    pub taxi_out_dur: Duration,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
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
