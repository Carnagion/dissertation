use std::{collections::HashMap, num::ParseIntError, time::Duration};

use chrono::ParseError;

use itertools::Itertools;

use thiserror::Error;

use runseq_instance::{
    flight::{Ctot, Deice, Departure, Flight},
    Instance,
};

mod flight;
use flight::{DeiceStatus, FlightId, FlightRow, ParseDeiceStatusError, ParseWeightClassError};

mod sep;
use sep::{create_separation_matrix, parse_separation_configs, SeparationConfigs};

const DATETIME_FMT: &'static str = "%Y-%m-%d %H:%M:%S";

const MINUTE: Duration = Duration::from_secs(60);

pub fn from_heathrow(
    flights: &str,
    pushback_durs: &str,
    separation_configs: &str,
) -> Result<Vec<Instance>, FromHeathrowError> {
    from_heathrow_with_limits(
        flights,
        pushback_durs,
        separation_configs,
        usize::MAX,
        usize::MAX,
    )
}

pub fn from_heathrow_with_limits(
    flights: &str,
    pushback_durs: &str,
    separation_configs: &str,
    instance_limit: usize,
    flight_limit: usize,
) -> Result<Vec<Instance>, FromHeathrowError> {
    let pushback_durs = parse_pushback_durs(pushback_durs)?;
    let separation_configs = parse_separation_configs(separation_configs)?;

    let instances = flights
        .lines()
        .map(FlightRow::parse)
        .process_results(|flights| {
            group_flights(
                flights,
                &pushback_durs,
                &separation_configs,
                instance_limit,
                flight_limit,
            )
        })?;

    Ok(instances)
}

fn group_flights<'a, 'f, F>(
    flights: F,
    pushback_durs: &'a HashMap<FlightId<'f>, Duration>,
    separation_configs: &'a SeparationConfigs<'f>,
    instance_limit: usize,
    flight_limit: usize,
) -> Vec<Instance>
where
    F: IntoIterator<Item = FlightRow<'f>>,
{
    let groups = flights.into_iter().group_by(|flight| flight.solved_at);
    groups
        .into_iter()
        .take(instance_limit)
        .map(|(_, group)| {
            let flight_rows = group.take(flight_limit).collect::<Vec<_>>();
            let separations = create_separation_matrix(&flight_rows, separation_configs);
            let flights = flight_rows
                .into_iter()
                .map(|flight| {
                    let pushback_duration = pushback_durs[&flight.aircraft_id];

                    let deice = match flight.deice_status {
                        DeiceStatus::None => None,
                        DeiceStatus::AtGates => Some(Deice {
                            duration: MINUTE * 5,
                            taxi_duration: Duration::ZERO,
                            hot: MINUTE * 15,
                        }),
                        DeiceStatus::AtApron => Some(Deice {
                            duration: MINUTE * 5,
                            taxi_duration: MINUTE * 5,
                            hot: MINUTE * 15,
                        }),
                    };

                    let taxi_duration = MINUTE * 5;
                    let lineup_duration = MINUTE * 5;

                    let ctot = flight.ctot.map(|target| Ctot {
                        target,
                        allow_early: MINUTE * 5,
                        allow_late: MINUTE * 10,
                    });

                    let mut earliest_time = flight.tobt + pushback_duration;
                    if let Some(deice) = &deice {
                        earliest_time += deice.taxi_duration;
                        earliest_time += deice.duration;
                    }
                    earliest_time += taxi_duration + lineup_duration;

                    Flight::Dep(Departure {
                        earliest_time,
                        base_time: earliest_time,
                        tobt: flight.tobt,
                        pushback_duration,
                        deice,
                        taxi_duration,
                        lineup_duration,
                        window: None,
                        ctot,
                    })
                })
                .collect::<Vec<_>>();
            Instance::new(flights, separations, MINUTE * 5).unwrap()
        })
        .collect::<Vec<_>>()
}

fn parse_pushback_durs<'a>(
    pushback_durs: &'a str,
) -> Result<HashMap<FlightId<'a>, Duration>, FromHeathrowError> {
    pushback_durs
        .lines()
        .map(|line| {
            let (aircraft_id, pushback_dur) =
                line.split_once(',').ok_or(FromHeathrowError::MissingData)?;
            let aircraft_id = FlightId(aircraft_id);
            let pushback_dur = pushback_dur
                .parse::<u64>()
                .map_err(FromHeathrowError::InvalidDuration)?;
            Ok((aircraft_id, Duration::from_secs(pushback_dur)))
        })
        .collect()
}

#[derive(Debug, Error)]
pub enum FromHeathrowError {
    #[error("missing one or more parts of data")]
    MissingData,
    #[error("invalid datetime: {}", .0)]
    InvalidDateTime(#[from] ParseError),
    #[error("invalid duration: {}", .0)]
    InvalidDuration(ParseIntError),
    #[error("invalid speed group: {}", .0)]
    InvalidSpeedGroup(ParseIntError),
    #[error(transparent)]
    InvalidWeightClass(#[from] ParseWeightClassError),
    #[error(transparent)]
    InvalidDeiceStatus(#[from] ParseDeiceStatusError),
}

fn next_part<'a, I>(parts: &mut I) -> Result<&'a str, FromHeathrowError>
where
    I: Iterator<Item = &'a str>,
{
    parts.next().ok_or(FromHeathrowError::MissingData)
}

fn extract_field<'a, I>(parts: &mut I, field: &str) -> Result<&'a str, FromHeathrowError>
where
    I: Iterator<Item = &'a str>,
{
    let part = next_part(parts)?;
    Ok(part
        .trim_start_matches(field)
        .trim_start()
        .trim_start_matches('=')
        .trim_start())
}

fn extract_opt_field<'a, I>(
    parts: &mut I,
    field: &str,
) -> Result<Option<&'a str>, FromHeathrowError>
where
    I: Iterator<Item = &'a str>,
{
    let field = extract_field(parts, field)?;
    match field {
        "(not set)" => Ok(None),
        field => Ok(Some(field)),
    }
}
