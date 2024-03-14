use std::time::Duration;

use chrono::NaiveDateTime;

use crate::heathrow::{
    extract_field,
    flight::RunwayId,
    next_part,
    FromHeathrowError,
    DATETIME_FMT,
};

pub struct RunwayConfig<'a> {
    solved_at: NaiveDateTime,
    runway_id: RunwayId<'a>,
    end_time: NaiveDateTime,
    min_sep: Duration,
    min_sep_on_change: Duration,
}

impl<'a> RunwayConfig<'a> {
    pub fn parse(data: &'a str) -> Result<Self, FromHeathrowError> {
        let mut parts = data.split(',').map(|part| part.trim());

        let solved_at = next_part(&mut parts)?;
        let solved_at = NaiveDateTime::parse_from_str(solved_at, DATETIME_FMT)?;

        let runway_id = extract_field(&mut parts, "Runway ID")?;

        let end_time = extract_field(&mut parts, "End Time")?;
        let end_time = NaiveDateTime::parse_from_str(end_time, DATETIME_FMT)?;

        let min_sep = extract_field(&mut parts, "MinimumRunwaySeparation")?
            .parse::<u64>()
            .map_err(FromHeathrowError::InvalidDuration)?;

        let min_sep_on_change = extract_field(&mut parts, "MinimumSeparationOnRunwayChange")?
            .parse::<u64>()
            .map_err(FromHeathrowError::InvalidDuration)?;

        Ok(Self {
            solved_at,
            runway_id: RunwayId(runway_id),
            end_time,
            min_sep: Duration::from_secs(min_sep),
            min_sep_on_change: Duration::from_secs(min_sep_on_change),
        })
    }
}
