use std::time::Duration;

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use thiserror::Error;

use crate::instance::{
    aircraft::Aircraft,
    duration::DurationMinutes,
    op::{ArrivalConstraints, DepartureConstraints, OpConstraints, OpKind},
};

#[serde_as] // NOTE: This must remain before the derive
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(try_from = "RawInstanceRow", into = "RawInstanceRow")]
pub struct InstanceRow {
    pub aircraft: Aircraft,
    pub constraints: OpConstraints,
    #[serde_as(as = "Vec<DurationMinutes>")]
    pub separations: Vec<Duration>,
}

impl TryFrom<RawInstanceRow> for InstanceRow {
    type Error = FromRawError;

    fn try_from(row: RawInstanceRow) -> Result<Self, Self::Error> {
        match row {
            RawInstanceRow {
                aircraft,
                op_kind: OpKind::Departure,
                earliest_time,
                pushback_dur: Some(pushback_dur),
                pre_de_ice_dur: Some(pre_de_ice_dur),
                de_ice_dur: Some(de_ice_dur),
                post_de_ice_dur: Some(post_de_ice_dur),
                lineup_dur: Some(lineup_dur),
                separations,
            } => {
                let constraints = DepartureConstraints {
                    earliest_time,
                    pushback_dur,
                    pre_de_ice_dur,
                    de_ice_dur,
                    post_de_ice_dur,
                    lineup_dur,
                };
                Ok(Self {
                    aircraft,
                    constraints: OpConstraints::Departure(constraints),
                    separations,
                })
            },
            RawInstanceRow {
                aircraft,
                op_kind: OpKind::Arrival,
                earliest_time,
                pushback_dur: None,
                pre_de_ice_dur: None,
                de_ice_dur: None,
                post_de_ice_dur: None,
                lineup_dur: None,
                separations,
            } => {
                let constraints = ArrivalConstraints { earliest_time };
                Ok(Self {
                    aircraft,
                    constraints: OpConstraints::Arrival(constraints),
                    separations,
                })
            },
            RawInstanceRow { op_kind, .. } => Err(FromRawError { op_kind }),
        }
    }
}

#[derive(Debug, Error)] // NOTE: No other traits implemented since this type will never be public
#[error("provided constraint data does not match data needed for {}", .op_kind)]
struct FromRawError {
    op_kind: OpKind,
}

#[serde_as] // NOTE: This must remain before the derive
#[derive(Debug, Deserialize, Serialize)] // NOTE: No other traits implemented since this type will never be public
struct RawInstanceRow {
    aircraft: Aircraft,
    op_kind: OpKind,
    earliest_time: NaiveTime,
    #[serde_as(as = "Option<DurationMinutes>")]
    pushback_dur: Option<Duration>,
    #[serde_as(as = "Option<DurationMinutes>")]
    pre_de_ice_dur: Option<Duration>,
    #[serde_as(as = "Option<DurationMinutes>")]
    de_ice_dur: Option<Duration>,
    #[serde_as(as = "Option<DurationMinutes>")]
    post_de_ice_dur: Option<Duration>,
    #[serde_as(as = "Option<DurationMinutes>")]
    lineup_dur: Option<Duration>,
    #[serde_as(as = "Vec<DurationMinutes>")]
    separations: Vec<Duration>,
}

impl From<InstanceRow> for RawInstanceRow {
    fn from(row: InstanceRow) -> Self {
        match row.constraints {
            OpConstraints::Departure(constraints) => Self {
                aircraft: row.aircraft,
                op_kind: OpKind::Departure,
                earliest_time: constraints.earliest_time,
                pushback_dur: Some(constraints.pushback_dur),
                pre_de_ice_dur: Some(constraints.pre_de_ice_dur),
                de_ice_dur: Some(constraints.de_ice_dur),
                post_de_ice_dur: Some(constraints.post_de_ice_dur),
                lineup_dur: Some(constraints.lineup_dur),
                separations: row.separations,
            },
            OpConstraints::Arrival(constraints) => Self {
                aircraft: row.aircraft,
                op_kind: OpKind::Arrival,
                earliest_time: constraints.earliest_time,
                pushback_dur: None,
                pre_de_ice_dur: None,
                de_ice_dur: None,
                post_de_ice_dur: None,
                lineup_dur: None,
                separations: row.separations,
            },
        }
    }
}
