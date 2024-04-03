use std::{num::ParseIntError, str::FromStr};

use chrono::NaiveDateTime;

use thiserror::Error;

use crate::heathrow::{extract_field, extract_opt_field, FromHeathrowError, DATETIME_FMT};

pub struct FlightRow<'a> {
    pub solved_at: NaiveDateTime,
    pub aircraft_id: FlightId<'a>,
    pub runway_id: Option<RunwayId<'a>>,
    pub stand_id: StandId<'a>,
    pub route_id: RouteId<'a>,
    pub speed_group: SpeedGroup,
    pub weight_class: WeightClass,
    pub atot: Option<NaiveDateTime>,
    pub ctot: Option<NaiveDateTime>,
    pub tobt: NaiveDateTime,
    pub aobt: Option<NaiveDateTime>,
    pub eczt: Option<NaiveDateTime>,
    pub deice_status: DeiceStatus,
}

impl<'a> FlightRow<'a> {
    pub fn parse(data: &'a str) -> Result<Self, FromHeathrowError> {
        let mut parts = data.split(',').map(|part| part.trim());

        let solved_at = parts.next().ok_or(FromHeathrowError::MissingData)?;
        let solved_at = NaiveDateTime::parse_from_str(solved_at, DATETIME_FMT)?;

        let aircraft_id = extract_field(&mut parts, "Aircraft ID")?;

        let runway_id = extract_opt_field(&mut parts, "Runway ID")?;

        let stand_id = extract_field(&mut parts, "Stand ID")?;

        let route_id = extract_field(&mut parts, "Route ID")?;

        let speed_group = extract_field(&mut parts, "Speed Group")?
            .parse::<u8>()
            .map_err(FromHeathrowError::InvalidSpeedGroup)?;

        let weight_class = extract_field(&mut parts, "Weight Class")?.parse::<WeightClass>()?;

        let atot = extract_opt_field(&mut parts, "ATOT")?
            .map(|atot| NaiveDateTime::parse_from_str(atot, DATETIME_FMT))
            .transpose()?;

        let ctot = extract_opt_field(&mut parts, "CTOT")?
            .map(|ctot| NaiveDateTime::parse_from_str(ctot, DATETIME_FMT))
            .transpose()?;

        let tobt = extract_field(&mut parts, "TOBT")?;
        let tobt = NaiveDateTime::parse_from_str(tobt, DATETIME_FMT)?;

        let aobt = extract_opt_field(&mut parts, "AOBT")?
            .map(|aobt| NaiveDateTime::parse_from_str(aobt, DATETIME_FMT))
            .transpose()?;

        let eczt = extract_opt_field(&mut parts, "ECZT")?
            .map(|eczt| NaiveDateTime::parse_from_str(eczt, DATETIME_FMT))
            .transpose()?;

        let deice_status = extract_field(&mut parts, "Deicing Status")?.parse::<DeiceStatus>()?;

        Ok(Self {
            solved_at,
            aircraft_id: FlightId(aircraft_id),
            runway_id: runway_id.map(RunwayId),
            stand_id: StandId(stand_id),
            route_id: RouteId(route_id),
            speed_group: SpeedGroup(speed_group),
            weight_class,
            atot,
            ctot,
            tobt,
            aobt,
            eczt,
            deice_status,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FlightId<'a>(pub &'a str);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RunwayId<'a>(pub &'a str);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct StandId<'a>(pub &'a str);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct RouteId<'a>(pub &'a str);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct SpeedGroup(pub u8);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WeightClass {
    Small,
    Medium,
    Heavy,
    Super,
    Upper,
}

impl FromStr for WeightClass {
    type Err = ParseWeightClassError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "Small" => Ok(Self::Small),
            "Medium" => Ok(Self::Medium),
            "Heavy" => Ok(Self::Heavy),
            "Super" => Ok(Self::Super),
            "Upper" => Ok(Self::Upper),
            _ => Err(ParseWeightClassError),
        }
    }
}

#[derive(Debug, Error)]
#[error("invalid weight class, expected `Small`, `Medium`, `Heavy`, `Super`, or `Upper`")]
pub struct ParseWeightClassError;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DeiceStatus {
    None,
    AtGates,
    AtApron,
}

impl FromStr for DeiceStatus {
    type Err = ParseDeiceStatusError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let status = string.parse()?;
        match status {
            0 => Ok(Self::None),
            1 => Ok(Self::AtGates),
            2 => Ok(Self::AtApron),
            status => Err(ParseDeiceStatusError::InvalidStatus(status)),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseDeiceStatusError {
    #[error("invalid de-ice status: {}", .0)]
    ParseNum(#[from] ParseIntError),
    #[error("invalid de-ice status `{}`, expected `0`, `1`, or `2`", .0)]
    InvalidStatus(u8),
}
