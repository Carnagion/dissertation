use std::{
    fmt::{self, Display, Formatter},
    time::Duration,
};

use chrono::NaiveTime;

use serde::{Deserialize, Serialize};

use serde_with::serde_as;

use crate::instance::duration::DurationMinutes;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OpKind {
    Departure,
    Arrival,
}

impl Display for OpKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let desc = match self {
            Self::Departure => "departure",
            Self::Arrival => "arrival",
        };
        write!(formatter, "{}", desc)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OpConstraints {
    Departure(DepartureConstraints),
    Arrival(ArrivalConstraints),
}

impl OpConstraints {
    pub fn kind(&self) -> OpKind {
        match self {
            Self::Departure(_) => OpKind::Departure,
            Self::Arrival(_) => OpKind::Arrival,
        }
    }

    pub fn earliest_time(&self) -> NaiveTime {
        let (Self::Departure(DepartureConstraints { earliest_time, .. })
        | Self::Arrival(ArrivalConstraints { earliest_time })) = self;
        *earliest_time
    }
}

impl From<DepartureConstraints> for OpConstraints {
    fn from(constraints: DepartureConstraints) -> Self {
        Self::Departure(constraints)
    }
}

impl From<ArrivalConstraints> for OpConstraints {
    fn from(constraints: ArrivalConstraints) -> Self {
        Self::Arrival(constraints)
    }
}

#[serde_as] // NOTE: This must remain before the derive
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)] // NOTE: We don't impl `Copy` since the struct's size is fairly large
#[serde(rename_all = "kebab-case")]
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
            - (self.lineup_dur
                + self.post_de_ice_dur
                + self.de_ice_dur
                + self.pre_de_ice_dur
                + self.pushback_dur)
    }

    pub fn target_de_ice_time(&self) -> NaiveTime {
        self.earliest_time - (self.lineup_dur + self.post_de_ice_dur + self.de_ice_dur)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)] // NOTE: We don't impl `Copy` to keep it consistent with `DepartureConstraints`
#[serde(rename_all = "kebab-case")]
pub struct ArrivalConstraints {
    pub earliest_time: NaiveTime,
}
