use std::time::Duration;

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
) -> impl Iterator<Item = Schedule> {
    match flight {
        Flight::Arr(arr) => {
            Left(expand_arrival(arr, flight_idx, instance, state).map(Schedule::Arr))
        },
        Flight::Dep(dep) => {
            Right(expand_departure(dep, flight_idx, instance, state).map(Schedule::Dep))
        },
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
        .then_some(landing)
        .into_iter()
        .map(move |landing| ArrivalSchedule {
            flight_idx: arr_idx,
            landing,
        })
}

fn expand_departure(
    dep: &Departure,
    dep_idx: usize,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = DepartureSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);
    let prev_dep_sched = state
        .current_solution
        .iter()
        .rev()
        .map(|node| &node.sched)
        .find_map(Schedule::as_departure);

    let (earliest_deice, latest_deice, takeoff) = match (prev_sched, prev_dep_sched) {
        (None, None) => {
            let takeoff = dep.release_time();

            let earliest_deice = (takeoff
                - instance.max_slack_dur
                - dep.lineup_dur
                - dep.taxi_out_dur
                - dep.deice_dur)
                .max(takeoff - instance.max_holdover_dur - dep.deice_dur);
            let latest_deice =
                (takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur).max(earliest_deice);

            (earliest_deice, latest_deice, takeoff)
        },
        (Some(prev_sched), None) => {
            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];

            let takeoff = dep.release_time().max(prev_sched.flight_time() + sep);

            let earliest_deice = (takeoff
                - instance.max_slack_dur
                - dep.lineup_dur
                - dep.taxi_out_dur
                - dep.deice_dur)
                .max(takeoff - instance.max_holdover_dur - dep.deice_dur);
            let latest_deice =
                (takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur).max(earliest_deice);

            (earliest_deice, latest_deice, takeoff)
        },
        (Some(prev_sched), Some(prev_dep_sched)) => {
            let prev_dep = instance.flights()[prev_dep_sched.flight_idx]
                .as_departure()
                .unwrap();
            let prev_deice_end = prev_dep_sched.deice + prev_dep.deice_dur;

            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];

            let takeoff = (prev_deice_end + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur)
                .max(dep.release_time())
                .max(prev_sched.flight_time() + sep);

            let earliest_deice = (takeoff
                - instance.max_slack_dur
                - dep.lineup_dur
                - dep.taxi_out_dur
                - dep.deice_dur)
                .max(takeoff - instance.max_holdover_dur - dep.deice_dur)
                .max(prev_deice_end);
            let latest_deice =
                (takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur).max(earliest_deice);

            (earliest_deice, latest_deice, takeoff)
        },
        // PANICS: This is fine, because there can never be a case where there is no immediately
        //         preceding flight but there is a preceding departure.
        (None, Some(_)) => unreachable!(),
    };

    dep.window
        .contains(takeoff)
        // NOTE: Since the last added node is explored first, the best node should be added last.
        //       Here, the first node leaves the least gaps in the de-icing queue, and is thus
        //       potentially the best and should be explored first. Reversing the iterator ensures this.
        .then(|| iter_minutes(earliest_deice, latest_deice).rev())
        .into_iter()
        .flatten()
        .map(move |deice| DepartureSchedule {
            flight_idx: dep_idx,
            deice,
            takeoff,
        })
}

fn iter_minutes(from: NaiveTime, to: NaiveTime) -> impl DoubleEndedIterator<Item = NaiveTime> {
    let diff = (to - from)
        .max(chrono::Duration::zero())
        .num_minutes()
        .unsigned_abs();
    (0..=diff).map(move |minute| from + Duration::from_secs(minute * 60))
}
