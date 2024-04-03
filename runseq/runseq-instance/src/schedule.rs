use chrono::NaiveDateTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Schedule {
    #[serde(rename = "arrival")]
    Arr(ArrivalSchedule),
    #[serde(rename = "departure")]
    Dep(DepartureSchedule),
}

impl Schedule {
    pub fn as_arrival(&self) -> Option<&ArrivalSchedule> {
        match self {
            Self::Arr(sched) => Some(sched),
            Self::Dep(_) => None,
        }
    }

    pub fn into_arrival(self) -> Option<ArrivalSchedule> {
        match self {
            Self::Arr(sched) => Some(sched),
            Self::Dep(_) => None,
        }
    }

    pub fn as_departure(&self) -> Option<&DepartureSchedule> {
        match self {
            Self::Dep(sched) => Some(sched),
            Self::Arr(_) => None,
        }
    }

    pub fn into_departure(self) -> Option<DepartureSchedule> {
        match self {
            Self::Dep(sched) => Some(sched),
            Self::Arr(_) => None,
        }
    }

    pub fn flight_index(&self) -> usize {
        match self {
            Self::Arr(arr) => arr.flight_index,
            Self::Dep(dep) => dep.flight_index,
        }
    }

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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ArrivalSchedule {
    pub flight_index: usize,
    pub landing: NaiveDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DepartureSchedule {
    pub flight_index: usize,
    pub deice: Option<NaiveDateTime>,
    pub takeoff: NaiveDateTime,
}
