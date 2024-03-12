use irdis_instance::{
    flight::{Arrival, Departure},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
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

    let violated = dep
        .ctot
        .as_ref()
        .is_some_and(|ctot| ctot.contains(sched.takeoff));
    let violation = if violated { VIOLATION_COST.pow(2) } else { 0 };

    let slack = (sched.takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur - sched.deice)
        .num_minutes()
        .unsigned_abs()
        .pow(2);

    delay + violation + slack
}

pub fn solution_cost(solution: &[Schedule], instance: &Instance) -> u64 {
    solution
        .iter()
        .map(|sched| match sched {
            Schedule::Arr(sched) => {
                let arr = instance.flights()[sched.flight_idx].as_arrival().unwrap();
                arrival_cost(sched, arr)
            },
            Schedule::Dep(sched) => {
                let dep = instance.flights()[sched.flight_idx].as_departure().unwrap();
                departure_cost(sched, dep)
            },
        })
        .sum::<u64>()
}
