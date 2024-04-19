//! Conversion from Milan Linate datasets.
//!
//! These problem instances are available on the University of Bologna's [online library of codes and instances](https://site.unibo.it/operations-research/en/research/library-of-codes-and-instances-1).

use std::{num::ParseIntError, time::Duration};

use chrono::{NaiveDateTime, ParseError};

use thiserror::Error;

use runseq_instance::{
    flight::{Arrival, Deice, Departure, Flight},
    sep::SeparationsLenError,
    Instance,
};

const MINUTE: Duration = Duration::from_secs(60);

/// Parses a problem instance from Milan Linate into an [`Instance`].
pub fn from_furini(flights: &str, separations: &str) -> Result<Instance, FromFuriniError> {
    from_furini_with_limit(flights, separations, usize::MAX)
}

/// Parses a problem instance from Milan Linate into an [`Instance`], stopping after a certain aircraft limit.
pub fn from_furini_with_limit(
    flights: &str,
    separations: &str,
    limit: usize,
) -> Result<Instance, FromFuriniError> {
    let mut lines = flights.lines();

    // NOTE: We could use this for pre-allocating the vec, but it's easier to `collect`.
    let flight_count = next_part(&mut lines)?.parse::<usize>()?.min(limit);

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
                .skip(1) // NOTE: We skip the first field since it's not a separation and is mostly unused.
                .take(limit)
                .map(|num| num.parse::<u64>().map(|dur| Duration::from_secs(dur * 60)))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(durations)
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    let separation_count = separations.len();

    Instance::new(flights, separations.try_into()?, MINUTE * 5).ok_or(
        FromFuriniError::MismatchedFlightSeparationsLen {
            flight_count,
            separation_count,
        },
    )
}

fn parse_flight(line: &str) -> Result<Flight, FromFuriniError> {
    let mut parts = line.split_ascii_whitespace();

    // NOTE: We don't actually need any of this data, but have to parse it nonetheless since the
    //       data is expected to be in a specific format.
    let _registration = next_part(&mut parts)?;
    let _model = next_part(&mut parts)?;
    let _size_class = next_part(&mut parts)?;

    let kind = next_part(&mut parts)?;

    let base_time = next_part(&mut parts)?;
    // NOTE: The Furini datasets don't include dates, only times, so we have to manually insert a date for `NaiveDateTime`
    //       to parse correctly. 19 April 2024 is chosen as it is the submission date for this dissertation.
    let earliest_time = format!("2024-04-19 {}", base_time);
    let earliest_time = NaiveDateTime::parse_from_str(&earliest_time, "%F %H%M")?;

    // NOTE: I don't actually know what this field is for. It's used in the separation
    //       matrix, but doesn't seem to serve any actual purpose.
    let _ = next_part(&mut parts)?;

    let flight = match kind {
        "A" => Ok(Flight::Arr(Arrival {
            earliest_time,
            base_time: earliest_time,
            window: None,
        })),
        "D" => Ok(Flight::Dep(Departure {
            earliest_time,
            base_time: earliest_time,
            tobt: earliest_time - MINUTE * 25,
            pushback_duration: MINUTE * 5,
            deice: Some(Deice {
                taxi_duration: MINUTE * 5,
                duration: MINUTE * 5,
                hot: MINUTE * 15,
            }),
            taxi_duration: MINUTE * 5,
            lineup_duration: MINUTE * 5,
            window: None,
            ctot: None,
        })),
        kind => Err(FromFuriniError::InvalidKind(kind.to_owned())),
    }?;

    Ok(flight)
}

/// The error returned when parsing a Milan problem instance fails.
#[derive(Debug, Error)]
pub enum FromFuriniError {
    /// One or more parts of the aircraft data is missing.
    #[error("missing one or more parts of data")]
    MissingData,
    /// The flight count could not be parsed.
    #[error("invalid flight count: {}", .0)]
    InvalidFlightCount(#[from] ParseIntError),
    /// The expected flight count does not match the actual number of aircraft in the data.
    #[error("mismatched flight count: expected {}, got {}", .expected, .actual)]
    MismatchedFlightCount {
        /// The expected flight count.
        expected: usize,
        /// The actual number of aircraft parsed.
        actual: usize,
    },
    /// A flight kind (arrival or departure) was invalid.
    #[error("invalid flight kind: {}", .0)]
    InvalidKind(String),
    /// The base time of an aircraft could not be parsed.
    #[error("invalid flight base time: {}", .0)]
    InvalidTime(#[from] ParseError),
    /// The separation matrix is not square.
    #[error("invalid separation matrix dimensions")]
    InvalidSeparationLen(#[from] SeparationsLenError),
    /// The dimensions of the separation matrix do not match the number of aircraft.
    #[error("flight count is {}, but separation matrix has {} rows and columns", .flight_count, .separation_count)]
    MismatchedFlightSeparationsLen {
        /// The number of aircraft.
        flight_count: usize,
        /// The number of aircraft according to the separation matrix.
        separation_count: usize,
    },
}

fn next_part<'a, I>(parts: &mut I) -> Result<&'a str, FromFuriniError>
where
    I: Iterator<Item = &'a str>,
{
    parts.next().ok_or(FromFuriniError::MissingData)
}
