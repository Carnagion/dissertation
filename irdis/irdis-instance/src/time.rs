use std::time::Duration;

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

mod duration;
pub(crate) use duration::{DurationMinutes, SeparationsAsMinutes};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TimeWindow {
    pub earliest: NaiveTime,
    pub latest: NaiveTime,
}

impl TimeWindow {
    pub fn contains(&self, time: NaiveTime) -> bool {
        (self.earliest..=self.latest).contains(&time)
    }
}

#[serde_as] // NOTE: This must remain before the derive
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Ctot {
    pub target: NaiveTime,
    #[serde_as(as = "DurationMinutes")]
    pub allow_before: Duration,
    #[serde_as(as = "DurationMinutes")]
    pub allow_after: Duration,
}

impl Ctot {
    pub fn earliest(&self) -> NaiveTime {
        self.target - self.allow_before
    }

    pub fn latest(&self) -> NaiveTime {
        self.target + self.allow_after
    }

    pub fn contains(&self, time: NaiveTime) -> bool {
        (self.earliest()..=self.latest()).contains(&time)
    }
}
