use chrono::NaiveTime;

use crate::instance::op::OpKind;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct RunwaySchedule(pub Vec<Op>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Op {
    pub aircraft_idx: usize,
    pub schedule: OpSchedule,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum OpSchedule {
    Departure(DepartureSchedule),
    Arrival(ArrivalSchedule),
}

impl OpSchedule {
    pub fn kind(&self) -> OpKind {
        match self {
            Self::Departure(_) => OpKind::Departure,
            Self::Arrival(_) => OpKind::Arrival,
        }
    }

    pub fn op_time(&self) -> NaiveTime {
        match self {
            Self::Departure(DepartureSchedule { take_off_time, .. }) => *take_off_time,
            Self::Arrival(ArrivalSchedule { landing_time }) => *landing_time,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DepartureSchedule {
    pub take_off_time: NaiveTime,
    pub de_ice_time: NaiveTime,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ArrivalSchedule {
    pub landing_time: NaiveTime,
}
