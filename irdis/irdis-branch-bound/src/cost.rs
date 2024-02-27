use irdis_instance::{
    flight::{Arrival, Departure},
    schedule::{ArrivalSchedule, DepartureSchedule},
};

// NOTE: Increasing the cost for violations has seemingly has no effect when
//       the objective function is set to using `earliest()` instead of `target`
//       (see below).
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
    // NOTE: Using the flight index does not fix the worsening of solutions from
    //       using `earliest()` instead of `target` (see below), but does make them
    //       slightly better. Maybe breaks symmetries?
    let violation = if arr.window.contains(&sched.landing) {
        sched.flight_idx as u64
    } else {
        VIOLATION_COST.pow(2)
    };

    // NOTE: Using `earliest()` instead of `target` makes solutions worse.
    let deviation = (sched.landing - arr.window.target)
        .num_minutes()
        .unsigned_abs()
        .pow(2);

    violation + deviation
}

pub fn departure_cost(sched: &DepartureSchedule, dep: &Departure) -> u64 {
    // NOTE: Using the flight index does not fix the worsening of solutions from
    //       using `earliest()` instead of `target` (see above), but does make them
    //       slightly better. Maybe breaks symmetries?
    let violation = if dep.ctot.contains(&sched.takeoff) {
        sched.flight_idx as u64
    } else {
        VIOLATION_COST.pow(2)
    };

    // NOTE: Unlike above, using `earliest()` instead of `target` here does NOT make solutions
    //       worse.
    let deviation = (sched.takeoff - dep.ctot.earliest())
        .num_minutes()
        .unsigned_abs()
        .pow(2);

    let slack = (sched.takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur - sched.deice)
        .num_minutes()
        .unsigned_abs();

    violation + deviation + slack
}
