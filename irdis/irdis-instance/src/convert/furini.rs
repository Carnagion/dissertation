use std::{num::ParseIntError, time::Duration};

use chrono::{NaiveTime, ParseError};

use thiserror::Error;

use crate::{
    flight::{Arrival, Departure, Flight},
    sep::SeparationsLenError,
    time::{Ctot, TimeWindow},
    Instance,
};

const MINUTE: Duration = Duration::from_secs(60);

impl Instance {
    pub fn from_furini(flights: &str, separations: &str) -> Result<Self, FromFuriniError> {
        Self::from_furini_with_limit(flights, separations, usize::MAX)
    }

    pub fn from_furini_with_limit(
        flights: &str,
        separations: &str,
        limit: usize,
    ) -> Result<Self, FromFuriniError> {
        let mut lines = flights.lines();

        // NOTE: We could use this for pre-allocating the vec, but it's easier to `collect`
        let flight_count = lines
            .next()
            .ok_or(FromFuriniError::MissingFlightCount)?
            .parse::<usize>()?
            .min(limit);

        let flights = lines
            .filter(|line| !line.is_empty())
            .take(limit)
            .map(parse_flight)
            .collect::<Result<Vec<_>, _>>()?;

        let flights = if flights.len() == flight_count {
            Ok(flights)
        } else {
            Err(FromFuriniError::MismatchedFlightCount {
                expected: flight_count,
                actual: flights.len(),
            })
        }?;

        let separations = separations
            .lines()
            .filter(|line| !line.is_empty())
            .take(limit)
            .map(|line| {
                let durations = line
                    .split_ascii_whitespace()
                    .skip(1)
                    .take(limit)
                    .map(|num| num.parse::<u64>().map(|dur| Duration::from_secs(dur * 60)))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(durations)
            })
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        let separation_count = separations.len();

        Self::new(flights, separations.try_into()?, MINUTE * 15, MINUTE * 5).ok_or(
            FromFuriniError::MismatchedFlightSeparationsLen {
                flight_count,
                separation_count,
            },
        )
    }
}

fn parse_flight(line: &str) -> Result<Flight, FromFuriniError> {
    let mut parts = line.split_ascii_whitespace();

    // NOTE: We don't actually need any of this data, but must parse it nonetheless since the
    //       data is expected to be in a specific format.
    let _registration = parts.next().ok_or(FromFuriniError::MissingRegistration)?;
    let _model = parts.next().ok_or(FromFuriniError::MissingModel)?;
    let _size_class = parts.next().ok_or(FromFuriniError::MissingSizeClass)?;

    let kind = parts.next().ok_or(FromFuriniError::MissingKind)?;

    let base_time = NaiveTime::parse_from_str(parts.next().unwrap(), "%H%M")?;

    // NOTE: I don't actually know what these numbers are for. They are used in the separation
    //       matrix, but doesn't seem to serve any actual purpose.
    let _unknown = parts.next().ok_or(FromFuriniError::MissingSepId)?;

    let flight = match kind {
        "A" => Ok(Flight::Arr(Arrival {
            base_time,
            window: TimeWindow {
                earliest: base_time,
                latest: base_time + MINUTE * 10,
            },
        })),
        "D" => Ok(Flight::Dep(Departure {
            base_time,
            window: TimeWindow {
                earliest: base_time,
                latest: base_time + MINUTE * 15,
            },
            ctot: Ctot {
                target: base_time + MINUTE * 5,
                allow_before: MINUTE * 5,
                allow_after: MINUTE * 10,
            },
            pushback_dur: MINUTE * 5,
            taxi_deice_dur: MINUTE * 5,
            deice_dur: MINUTE * 5,
            taxi_out_dur: MINUTE * 5,
            lineup_dur: MINUTE * 5,
        })),
        kind => Err(FromFuriniError::InvalidKind(kind.to_owned())),
    }?;

    Ok(flight)
}

#[derive(Debug, Error)]
pub enum FromFuriniError {
    #[error("missing flight count")]
    MissingFlightCount,
    #[error("invalid flight count: {}", .0)]
    InvalidFlightCount(#[from] ParseIntError),
    #[error("mismatched flight count: expected {}, got {}", .expected, .actual)]
    MismatchedFlightCount { expected: usize, actual: usize },
    #[error("missing flight registration number")]
    MissingRegistration,
    #[error("missing flight model")]
    MissingModel,
    #[error("missing flight size class")]
    MissingSizeClass,
    #[error("missing flight kind")]
    MissingKind,
    #[error("invalid flight kind: {}", .0)]
    InvalidKind(String),
    #[error("invalid flight base time: {}", .0)]
    InvalidTime(#[from] ParseError),
    #[error("missing flight separation ID")]
    MissingSepId,
    #[error("invalid separation matrix dimensions")]
    InvalidSeparationLen(#[from] SeparationsLenError),
    #[error("flight count is {}, but separation matrix has {} rows and columns", .flight_count, .separation_count)]
    MismatchedFlightSeparationsLen {
        flight_count: usize,
        separation_count: usize,
    },
}
