use chrono::NaiveTime;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Departure {
    pub aircraft_idx: usize,
    pub de_ice_time: NaiveTime,
    pub take_off_time: NaiveTime,
}
