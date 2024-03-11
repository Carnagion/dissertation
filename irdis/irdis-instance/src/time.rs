use std::time::Duration;

#[cfg(feature = "serde")]
use cfg_eval::cfg_eval;

use chrono::NaiveTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_with::serde_as;

#[cfg(feature = "serde")]
mod duration;

#[cfg(feature = "serde")]
pub(crate) use duration::{DurationMinutes, SeparationsAsMinutes};

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TimeWindow {
    pub earliest: NaiveTime,
    pub latest: NaiveTime,
}

impl TimeWindow {
    pub fn contains(&self, time: NaiveTime) -> bool {
        (self.earliest..=self.latest).contains(&time)
    }
}

// NOTE: This must remain before the derive. The `cfg_eval` is to make the inner `cfg_attr` attributes
//       evaluate before `serde_as` is applied, which allows `serde_as` to function properly.
#[cfg_attr(feature = "serde", cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ctot {
    pub target: NaiveTime,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
    pub allow_before: Duration,
    #[cfg_attr(feature = "serde", serde_as(as = "DurationMinutes"))]
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
