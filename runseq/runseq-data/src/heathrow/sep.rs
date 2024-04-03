use std::{collections::HashMap, time::Duration};

use runseq_instance::sep::Separations;

use crate::heathrow::{
    flight::{FlightRow, RouteId, RunwayId, SpeedGroup, WeightClass},
    next_part,
    FromHeathrowError,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SeparationFactors<'a> {
    runway_id: RunwayId<'a>,
    weight_class: WeightClass,
    route_id: RouteId<'a>,
    speed_group: SpeedGroup,
}

fn parse_separation_factors<'a, I>(
    parts: &mut I,
) -> Result<SeparationFactors<'a>, FromHeathrowError>
where
    I: Iterator<Item = &'a str>,
{
    let runway_id = next_part(parts)?;
    let weight_class = next_part(parts)?.parse()?;
    let route_id = next_part(parts)?;
    let speed_group = next_part(parts)?
        .parse()
        .map_err(FromHeathrowError::InvalidSpeedGroup)?;
    Ok(SeparationFactors {
        runway_id: RunwayId(runway_id),
        weight_class,
        route_id: RouteId(route_id),
        speed_group: SpeedGroup(speed_group),
    })
}

pub type SeparationConfigs<'a> = HashMap<(SeparationFactors<'a>, SeparationFactors<'a>), Duration>;

pub fn parse_separation_configs(
    separation_configs: &str,
) -> Result<SeparationConfigs<'_>, FromHeathrowError> {
    separation_configs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|part| part.trim());

            let from = parse_separation_factors(&mut parts)?;
            let to = parse_separation_factors(&mut parts)?;

            let sep = next_part(&mut parts)?;
            let sep = sep.parse().map_err(FromHeathrowError::InvalidDuration)?;

            Ok(((from, to), Duration::from_secs(sep)))
        })
        .collect()
}

const MINUTE: Duration = Duration::from_secs(60);

pub fn create_separation_matrix(
    flights: &[FlightRow<'_>],
    separation_configs: &SeparationConfigs<'_>,
) -> Separations {
    let mut separations = vec![vec![Duration::ZERO; flights.len()]; flights.len()];
    for (idx_from, from) in flights.iter().enumerate() {
        for (idx_to, to) in flights.iter().enumerate() {
            let sep = match from.runway_id.zip(to.runway_id) {
                None => MINUTE,
                Some((runway_from, runway_to)) => {
                    let from = SeparationFactors {
                        runway_id: runway_from,
                        weight_class: from.weight_class,
                        route_id: from.route_id,
                        speed_group: from.speed_group,
                    };
                    let to = SeparationFactors {
                        runway_id: runway_to,
                        weight_class: to.weight_class,
                        route_id: to.route_id,
                        speed_group: to.speed_group,
                    };
                    separation_configs
                        .get(&(from, to))
                        .copied()
                        .unwrap_or(MINUTE)
                },
            };
            separations[idx_from][idx_to] = sep;
        }
    }
    separations.try_into().unwrap()
}
