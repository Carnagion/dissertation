use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

use runseq_instance::{
    flight::{Arrival, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::search::BranchBoundState;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Cost {
    pub delay: u64,
    pub ctot_violation: u64,
    pub runway_hold: u64,
}

impl Cost {
    pub const MAX: Self = Self {
        delay: u64::MAX,
        ctot_violation: 0,
        runway_hold: 0,
    };

    pub fn as_u64(&self) -> u64 {
        self.delay + self.ctot_violation
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_u64()
            .cmp(&other.as_u64())
            .then_with(|| self.runway_hold.cmp(&other.runway_hold))
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            delay: self.delay + rhs.delay,
            ctot_violation: self.ctot_violation + rhs.ctot_violation,
            runway_hold: self.runway_hold + rhs.runway_hold,
        }
    }
}

impl AddAssign for Cost {
    fn add_assign(&mut self, rhs: Self) {
        self.delay += rhs.delay;
        self.ctot_violation += rhs.ctot_violation;
        self.runway_hold += rhs.runway_hold;
    }
}

impl Sub for Cost {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            delay: self.delay - rhs.delay,
            ctot_violation: self.ctot_violation - rhs.ctot_violation,
            runway_hold: self.runway_hold - rhs.runway_hold,
        }
    }
}

impl SubAssign for Cost {
    fn sub_assign(&mut self, rhs: Self) {
        self.delay -= rhs.delay;
        self.ctot_violation -= rhs.ctot_violation;
        self.runway_hold -= rhs.runway_hold;
    }
}

impl Sum for Cost {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |sum, cost| sum + cost)
    }
}

pub fn arrival_cost(sched: &ArrivalSchedule, arr: &Arrival) -> Cost {
    let delay = (sched.landing - arr.base_time)
        .num_seconds()
        .unsigned_abs()
        .pow(2);

    Cost {
        delay,
        ctot_violation: 0,
        runway_hold: 0,
    }
}

pub fn departure_cost(sched: &DepartureSchedule, dep: &Departure) -> Cost {
    let delay = (sched.takeoff - dep.base_time)
        .num_seconds()
        .unsigned_abs()
        .pow(2);

    let ctot_violation = match &dep.ctot {
        None => 0,
        Some(ctot) if ctot.as_range().contains(&sched.takeoff) => 0,
        Some(ctot) => (sched.takeoff - ctot.latest())
            .num_seconds()
            .unsigned_abs()
            .pow(2),
    };

    let runway_hold = match sched.deice {
        None => 0,
        Some(deice) => {
            // The runway hold time can be calculated as the difference between the departure's latest possible
            // de-icing time (according to its scheduled take-off time) and its actual de-icing time.
            let deice_duration = dep.deice.as_ref().unwrap().duration;
            let runway_hold =
                sched.takeoff - dep.lineup_duration - dep.taxi_duration - deice_duration - deice;
            runway_hold.num_seconds().unsigned_abs().pow(2)
        },
    };

    Cost {
        delay,
        ctot_violation,
        runway_hold,
    }
}

pub fn schedule_cost(sched: &Schedule, instance: &Instance) -> Cost {
    match sched {
        Schedule::Arr(sched) => {
            let arr = instance.flights()[sched.flight_index].as_arrival().unwrap();
            arrival_cost(sched, arr)
        },
        Schedule::Dep(sched) => {
            let dep = instance.flights()[sched.flight_index]
                .as_departure()
                .unwrap();
            departure_cost(sched, dep)
        },
    }
}

/// Calculates the objective value of a runway sequence.
///
/// # Panics
///
/// This function will panic if the number of aircraft in the sequence does not match the number of aircraft in the instance,
/// which can happen if the given runway sequence was not produced by solving the given instance.
pub fn solution_cost(solution: &[Schedule], instance: &Instance) -> Cost {
    solution
        .iter()
        .map(|sched| schedule_cost(sched, instance))
        .sum()
}

pub fn estimated_remaining_cost(
    instance: &Instance,
    state: &BranchBoundState,
    last_sched: &Schedule,
) -> Cost {
    // NOTE: A minimum separation of zero seconds is used as this seems to provide better lower bounds.
    let min_sep = Duration::from_secs(0);

    // To calculate the estimated remaining cost, every remaining aircraft is scheduled as soon as possible,
    // assuming a minimum separation as above.
    // A quirk of this method is that first aircraft from each set of complete-ordered aircraft may all assumed to
    // be scheduled at the same time.
    // However, this is not an issue, as we only need to calculate a lower bound on the cost of scheduling aircraft
    // in a quick and cheap way.
    // Additionally, this method ensures that the lower bound will never exceed the actual cost, since when actually
    // scheduling aircraft they will never be scheduled to land or take-off at the same time.
    state
        .complete_order_sets
        .iter()
        .zip(&state.next_in_complete_order_sets)
        .map(|(complete_order_set, &next_in_set_idx)| &complete_order_set[next_in_set_idx..])
        .map(|remaining_solution| {
            remaining_solution
                .iter()
                .scan(last_sched.clone(), |last_sched, &flight_idx| {
                    let flight = &instance.flights()[flight_idx];
                    let sched = match flight {
                        Flight::Arr(arr) => Schedule::Arr(ArrivalSchedule {
                            flight_index: flight_idx,
                            landing: arr.release_time().max(last_sched.flight_time() + min_sep),
                        }),
                        Flight::Dep(dep) => Schedule::Dep(DepartureSchedule {
                            flight_index: flight_idx,
                            takeoff: dep.release_time().max(last_sched.flight_time() + min_sep),
                            deice: None,
                        }),
                    };
                    *last_sched = sched.clone();
                    Some(sched)
                })
        })
        .flat_map(|remaining_solution| {
            remaining_solution.map(|sched| schedule_cost(&sched, instance))
        })
        .sum()
}
