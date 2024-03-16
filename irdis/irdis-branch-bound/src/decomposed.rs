use std::{collections::HashMap, iter};

use chrono::NaiveTime;

use either::{Left, Right};

use irdis_instance::{
    flight::{Arrival, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::BranchBoundState;

pub fn expand(
    flight: &Flight,
    flight_idx: usize,
    instance: &Instance,
    state: &BranchBoundState,
    deice_queue: &HashMap<usize, NaiveTime>,
) -> impl Iterator<Item = Schedule> {
    match flight {
        Flight::Arr(arr) => {
            Left(expand_arrival(arr, flight_idx, instance, state).map(Schedule::Arr))
        },
        Flight::Dep(dep) => Right(
            expand_departure(dep, flight_idx, instance, state, deice_queue).map(Schedule::Dep),
        ),
    }
}

fn expand_arrival(
    arr: &Arrival,
    arr_idx: usize,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = ArrivalSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);

    let landing = match prev_sched {
        None => arr.release_time(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), arr_idx)];
            arr.release_time().max(prev_sched.flight_time() + sep)
        },
    };

    arr.window
        .contains(landing)
        .then_some(ArrivalSchedule {
            flight_idx: arr_idx,
            landing,
        })
        .into_iter()
}

fn expand_departure(
    dep: &Departure,
    dep_idx: usize,
    instance: &Instance,
    state: &BranchBoundState,
    deice_queue: &HashMap<usize, NaiveTime>,
) -> impl Iterator<Item = DepartureSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);

    let (deice, takeoff) = match prev_sched {
        None => {
            let deice = deice_queue[&dep_idx];
            let takeoff =
                (deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur).max(dep.release_time());

            (deice, takeoff)
        },
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];

            let deice = deice_queue[&dep_idx];
            let takeoff = (deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur)
                .max(dep.release_time())
                .max(prev_sched.flight_time() + sep);

            (deice, takeoff)
        },
    };

    (dep.window.contains(takeoff) && takeoff <= deice + dep.deice_dur + instance.max_holdover_dur)
        .then_some(DepartureSchedule {
            flight_idx: dep_idx,
            deice,
            takeoff,
        })
        .into_iter()
}

pub fn deice_queue(instance: &Instance) -> Option<HashMap<usize, NaiveTime>> {
    let mut departures = instance
        .flights()
        .iter()
        .enumerate()
        .filter_map(|(idx, flight)| flight.as_departure().zip(Some(idx)))
        .collect::<Vec<_>>();
    departures.sort_unstable_by_key(|(dep, _)| dep.release_time());

    let &(first_dep, first_dep_idx) = departures.first()?;
    let first_deice = (first_dep.release_time()
        - first_dep.lineup_dur
        - first_dep.taxi_out_dur
        - first_dep.deice_dur)
        .max(first_dep.release_time() - instance.max_holdover_dur - first_dep.deice_dur);
    let first_sched = (first_dep_idx, first_deice);

    let deice_queue = departures
        .into_iter()
        .skip(1) // NOTE: We scheduled the first departure's de-ice time above already, so we can skip it
        .scan(first_sched, |(prev_dep_idx, prev_deice), (dep, dep_idx)| {
            let prev_dep = instance.flights()[*prev_dep_idx].as_departure().unwrap();

            let deice = (dep.release_time()
                - dep.lineup_dur
                - dep.taxi_out_dur
                - dep.deice_dur)
                .max(*prev_deice + prev_dep.deice_dur)
                .max(dep.release_time() - instance.max_holdover_dur - first_dep.deice_dur);

            *prev_deice = deice;
            *prev_dep_idx = dep_idx;

            Some((dep_idx, deice))
        });

    Some(iter::once(first_sched).chain(deice_queue).collect())
}
