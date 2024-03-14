use std::{collections::HashMap, time::Duration};

use crate::heathrow::{
    flight::{FlightId, RunwayId, StandId},
    next_part,
    FromHeathrowError,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TaxiFactors<'a> {
    aircraft_id: FlightId<'a>,
    stand_id: StandId<'a>,
    runway_id: RunwayId<'a>,
}

pub type TaxiConfigs<'a> = HashMap<TaxiFactors<'a>, Duration>;

pub fn parse_taxi_configs(taxi_configs: &str) -> Result<TaxiConfigs<'_>, FromHeathrowError> {
    taxi_configs
        .lines()
        .map(|line| {
            let mut parts = line.split(',').map(|part| part.trim());

            let aircraft_id = next_part(&mut parts)?;
            let stand_id = next_part(&mut parts)?;
            let runway_id = next_part(&mut parts)?;

            let pushback_dur = next_part(&mut parts)?
                .parse()
                .map_err(FromHeathrowError::InvalidDuration)?;

            let taxi = TaxiFactors {
                aircraft_id: FlightId(aircraft_id),
                stand_id: StandId(stand_id),
                runway_id: RunwayId(runway_id),
            };

            Ok((taxi, Duration::from_secs(pushback_dur)))
        })
        .collect()
}
