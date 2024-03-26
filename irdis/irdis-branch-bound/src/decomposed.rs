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
    let deice = deice_queue[&dep_idx];

    let prev_sched = state.current_solution.last().map(|node| &node.sched);
    let takeoff = match prev_sched {
        None => {
            let takeoff =
                (deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur).max(dep.release_time());
            takeoff
        },
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];

            let takeoff = (deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur)
                .max(dep.release_time())
                .max(prev_sched.flight_time() + sep);
            takeoff
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

pub fn deice_queue<K, F>(instance: &Instance, mut sort_by: F) -> Option<HashMap<usize, NaiveTime>>
where
    F: FnMut(&Departure) -> K,
    K: Ord,
{
    let mut departures = instance
        .flights()
        .iter()
        .enumerate()
        .filter_map(|(idx, flight)| flight.as_departure().zip(Some(idx)))
        .collect::<Vec<_>>();
    departures.sort_unstable_by_key(|(dep, _)| sort_by(dep));

    let mut departures = departures.drain(..);

    let (first_dep, first_dep_idx) = departures.next()?;
    let first_deice = (first_dep.release_time()
        - first_dep.lineup_dur
        - first_dep.taxi_out_dur
        - first_dep.deice_dur)
        .max(first_dep.release_time() - instance.max_holdover_dur - first_dep.deice_dur);
    let first_sched = (first_dep_idx, first_deice);

    let deice_queue = departures.scan(first_sched, |(prev_dep_idx, prev_deice), (dep, dep_idx)| {
        let prev_dep = instance.flights()[*prev_dep_idx].as_departure().unwrap();

        let deice = (dep.release_time() - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur)
            .max(*prev_deice + prev_dep.deice_dur)
            .max(dep.release_time() - instance.max_holdover_dur - first_dep.deice_dur);

        *prev_deice = deice;
        *prev_dep_idx = dep_idx;

        Some((dep_idx, deice))
    });

    Some(iter::once(first_sched).chain(deice_queue).collect())
}
