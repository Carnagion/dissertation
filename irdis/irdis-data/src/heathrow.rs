use std::{collections::HashMap, num::ParseIntError, time::Duration};

use chrono::ParseError;

use itertools::Itertools;

use thiserror::Error;

use irdis_instance::{
    flight::{Departure, Flight},
    time::{Ctot, TimeWindow},
    Instance,
};

mod flight;
use flight::{DeiceStatus, FlightId, FlightRow, ParseDeiceStatusError, ParseWeightClassError};

mod taxi;
use taxi::{parse_taxi_configs, TaxiConfigs};

mod sep;
use sep::{create_separation_matrix, parse_separation_configs, SeparationConfigs};

mod runway;
use runway::RunwayConfig;

const DATETIME_FMT: &'static str = "%Y-%m-%d %H:%M:%S";

const MINUTE: Duration = Duration::from_secs(60);

pub fn from_heathrow(
    flights: &str,
    pushback_durs: &str,
    taxi_configs: &str,
    runway_configs: &str,
    separation_configs: &str,
) -> Result<Vec<Instance>, FromHeathrowError> {
    from_heathrow_with_limit(
        flights,
        pushback_durs,
        taxi_configs,
        runway_configs,
        separation_configs,
        usize::MAX,
    )
}

pub fn from_heathrow_with_limit(
    flights: &str,
    pushback_durs: &str,
    taxi_configs: &str,
    runway_configs: &str,
    separation_configs: &str,
    limit: usize,
) -> Result<Vec<Instance>, FromHeathrowError> {
    let runway_configs = runway_configs
        .lines()
        .map(RunwayConfig::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let pushback_durs = parse_pushback_durs(pushback_durs)?;
    let taxi_configs = parse_taxi_configs(taxi_configs)?;
    let separation_configs = parse_separation_configs(separation_configs)?;

    let instances = flights
        .lines()
        .map(FlightRow::parse)
        .process_results(|flights| {
            group_flights(
                flights,
                &pushback_durs,
                &taxi_configs,
                &runway_configs,
                &separation_configs,
                limit,
            )
        })?;

    Ok(instances)
}

fn group_flights<'a, 'f, F>(
    flights: F,
    pushback_durs: &'a HashMap<FlightId<'f>, Duration>,
    taxi_configs: &'a TaxiConfigs<'f>,
    runway_configs: &'a [RunwayConfig<'f>],
    separation_configs: &'a SeparationConfigs<'f>,
    limit: usize,
) -> Vec<Instance>
where
    F: IntoIterator<Item = FlightRow<'f>>,
{
    let groups = flights.into_iter().group_by(|flight| flight.solved_at);
    groups
        .into_iter()
        .take(limit)
        .map(|(_, group)| {
            let flight_rows = group.collect::<Vec<_>>();
            let separations = create_separation_matrix(&flight_rows, separation_configs);
            let flights = flight_rows
                .into_iter()
                .map(|flight| {
                    let pushback_dur = pushback_durs[&flight.aircraft_id];
                    let (taxi_deice_dur, taxi_out_dur) = match flight.deice_status {
                        DeiceStatus::None => (Duration::ZERO, Duration::ZERO),
                        DeiceStatus::AtGates => (Duration::ZERO, MINUTE * 5),
                        DeiceStatus::AtApron => (MINUTE * 5, MINUTE * 5),
                    };
                    let deice_dur = MINUTE * 5;
                    let lineup_dur = MINUTE * 5;

                    let base_time = flight.tobt.time()
                        + pushback_dur
                        + taxi_deice_dur
                        + deice_dur
                        + taxi_out_dur
                        + lineup_dur;

                    let ctot = flight.ctot.map(|target| Ctot {
                        target: target.time(),
                        allow_before: MINUTE * 5,
                        allow_after: MINUTE * 10,
                    });

                    Flight::Dep(Departure {
                        tobt: flight.tobt.time(),
                        window: TimeWindow {
                            earliest: base_time,
                            latest: base_time + MINUTE * 60,
                        },
                        ctot,
                        pushback_dur,
                        taxi_deice_dur,
                        deice_dur: MINUTE * 5,
                        taxi_out_dur,
                        lineup_dur: MINUTE * 5,
                    })
                })
                .collect::<Vec<_>>();
            Instance::new(flights, separations, MINUTE * 15, MINUTE * 5).unwrap()
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
