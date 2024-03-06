use irdis_instance::{
    flight::{Arrival, Departure},
    schedule::{ArrivalSchedule, DepartureSchedule},
};

const VIOLATION_COST: u64 = 60;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Cost {
    pub current: u64,
    pub lowest: u64,
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            current: 0,
            lowest: u64::MAX,
        }
    }
}

pub fn arrival_cost(sched: &ArrivalSchedule, arr: &Arrival) -> u64 {
    let delay = (sched.landing - arr.base_time)
        .num_minutes()
        .unsigned_abs()
        .pow(2);
    delay
}

pub fn departure_cost(sched: &DepartureSchedule, dep: &Departure) -> u64 {
    let delay = (sched.takeoff - dep.base_time)
        .num_minutes()
        .unsigned_abs()
        .pow(2);

    let violation = if dep.ctot.contains(sched.takeoff) {
        0
    } else {
        VIOLATION_COST.pow(2)
    };

    let slack = (sched.takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur - sched.deice)
        .num_minutes()
        .unsigned_abs()
        .pow(1); // TODO: Change this back to `pow(2)` once the CPLEX model has done the same

    delay + violation + slack
}
