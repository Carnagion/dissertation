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
    let min_sep = Duration::from_secs(0);

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

    // let mut next_offsets = vec![0; state.next_in_complete_order_sets.len()];

    // let mut schedule_estimate =
    //     Vec::with_capacity(instance.flights().len() - state.current_solution.len() - 1);

    // let mut cost_estimate = Cost::default();

    // while let Some(flight_idx) =
    //     estimate_next_flight(instance, state, last_sched, &schedule_estimate)
    // {
    //     schedule_estimate.push(flight_idx);
    // }
}

// fn estimate_next_flight(
//     instance: &Instance,
//     state: &BranchBoundState,
//     last_sched: &Schedule,
//     schedule_estimate: &[usize],
// ) -> Option<usize> {
//     state
//         .complete_order_sets
//         .iter()
//         .zip(&state.next_in_complete_order_sets)
//         .filter_map(|(complete_order_set, &next_in_set_idx)| {
//             let next_flight_idxs = complete_order_set.get(next_in_set_idx..)?;
//             next_flight_idxs.iter().find(|&&flight_idx| {
//                 flight_idx != last_sched.flight_index() && !schedule_estimate.contains(&flight_idx)
//             })
//         })
//         .min_by_key(|&&flight_idx| instance.flights()[flight_idx].release_time())
//         .copied()
// }
