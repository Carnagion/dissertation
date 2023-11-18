use std::time::Duration;

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use crate::instance::duration::DurationMinutes;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Op {
    Arrival,
    Departure,
}

#[serde_as] // NOTE: This must remain before the derive
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)] // NOTE: Theoretically could be Copy but is quite big
pub struct OpConstraints {
    pub earliest_time: NaiveTime,
    #[serde_as(as = "DurationMinutes")]
    pub pushback_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub pre_de_ice_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub de_ice_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub post_de_ice_dur: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub lineup_dur: Duration,
}
