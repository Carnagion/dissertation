use irdis_instance::{
    flight::{Arrival, Departure},
    schedule::{ArrivalSchedule, DepartureSchedule},
};

const VIOLATION_COST: u64 = 60;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cost {
    pub current: u64,
    pub best: u64,
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            current: 0,
            best: u64::MAX,
        }
    }
}

pub fn arrival_cost(sched: &ArrivalSchedule, arr: &Arrival) -> u64 {
    let violation = if arr.window.contains(&sched.landing) {
        0
    } else {
        VIOLATION_COST.pow(2)
    };

    let deviation = (sched.landing - arr.window.target)
        .num_minutes()
        .unsigned_abs()
        .pow(2);

    violation + deviation
}

pub fn departure_cost(sched: &DepartureSchedule, dep: &Departure) -> u64 {
    let violation = if dep.ctot.contains(&sched.takeoff) {
        0
    } else {
        VIOLATION_COST.pow(2)
    };

    let deviation = (sched.takeoff - dep.ctot.target)
        .num_minutes()
        .unsigned_abs()
        .pow(2);

    let slack = (sched.takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur - sched.deice)
        .num_minutes()
        .unsigned_abs();

    violation + deviation + slack
}
