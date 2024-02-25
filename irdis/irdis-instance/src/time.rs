use std::time::Duration;

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

mod duration;
pub(crate) use duration::{DurationMinutes, SeparationsAsMinutes};

#[serde_as] // NOTE: This must remain before the derive
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TimeWindow {
    pub target: NaiveTime,
    #[serde_as(as = "DurationMinutes")]
    pub before: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub after: Duration,
}

impl TimeWindow {
    pub fn earliest(&self) -> NaiveTime {
        self.target - self.before
    }

    pub fn latest(&self) -> NaiveTime {
        self.target + self.after
    }
}
