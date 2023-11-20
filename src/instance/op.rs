use std::time::Duration;

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use crate::instance::duration::DurationMinutes;

#[serde_as] // NOTE: This must remain before the derive
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)] // NOTE: Theoretically could be Copy but is quite big
pub struct DepartureConstraints {
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

impl DepartureConstraints {
    pub fn target_off_block_time(&self) -> NaiveTime {
        self.earliest_time
            - (self.pushback_dur
                + self.pre_de_ice_dur
                + self.de_ice_dur
                + self.post_de_ice_dur
                + self.lineup_dur)
    }
}
