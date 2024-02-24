use chrono::NaiveTime;

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
    pub fn flight_index(&self) -> usize {
        match self {
            Self::Arr(sched) => sched.flight_idx,
            Self::Dep(sched) => sched.flight_idx,
        }
    }

    pub fn flight_time(&self) -> NaiveTime {
        match self {
            Self::Arr(sched) => sched.landing,
            Self::Dep(sched) => sched.takeoff,
        }
    }

    pub fn as_arrival(&self) -> Option<&ArrivalSchedule> {
        match self {
            Self::Arr(sched) => Some(sched),
            _ => None,
        }
    }

    pub fn into_arrival(self) -> Option<ArrivalSchedule> {
        match self {
            Self::Arr(sched) => Some(sched),
            _ => None,
        }
    }

    pub fn as_departure(&self) -> Option<&DepartureSchedule> {
        match self {
            Self::Dep(sched) => Some(sched),
            _ => None,
        }
    }

    pub fn into_departure(self) -> Option<DepartureSchedule> {
        match self {
            Self::Dep(sched) => Some(sched),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ArrivalSchedule {
    pub flight_idx: usize,
    pub landing: NaiveTime,
}

impl From<ArrivalSchedule> for Schedule {
    fn from(sched: ArrivalSchedule) -> Self {
        Self::Arr(sched)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DepartureSchedule {
    pub flight_idx: usize,
    pub deice: NaiveTime,
    pub takeoff: NaiveTime,
}

impl From<DepartureSchedule> for Schedule {
    fn from(sched: DepartureSchedule) -> Self {
        Self::Dep(sched)
    }
}
