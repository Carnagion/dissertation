use std::num::NonZeroUsize;

use either::{Left, Right};

use runseq_instance::{
    flight::{Arrival, Deice, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::search::{branch_bound, iter_minutes, within_window, BranchBoundState};

pub fn branch_bound_rolling(
    instance: &Instance,
    horizon: Option<NonZeroUsize>,
) -> Option<Vec<Schedule>> {
    let flight_count = instance.flights().len();

    let mut state = BranchBoundState::new(instance);

    let end = horizon
        .map(usize::from)
        .unwrap_or(flight_count)
        .min(flight_count);

    let mut nodes = Vec::with_capacity(flight_count);

    branch_bound(instance, &mut state, &mut nodes, &mut expand, 0..end);

    let windows = (1..)
        .zip(end + 1..=flight_count)
        .map(|(start, end)| start..end);
    for window in windows {
        let fixed = state.best_solution.drain(..).next()?;
        let fixed_idx = fixed.sched.flight_index();

        state.current_solution.push(fixed);

        state.next_in_complete_order_sets.fill(0);
        for set in &mut state.complete_order_sets {
            set.retain(|&flight_idx| flight_idx != fixed_idx);
        }

        branch_bound(instance, &mut state, &mut nodes, &mut expand, window);
    }

    let last_best_solution = (!state.best_solution.is_empty()).then_some(state.best_solution)?;

    let solution = state
        .current_solution
        .into_iter()
        .chain(last_best_solution)
        .map(|node| node.sched)
        .collect();
    Some(solution)
}

fn expand(
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
    flight_index: usize,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = ArrivalSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);

    let landing = match prev_sched {
        None => arr.release_time(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), flight_index)];
            arr.release_time().max(prev_sched.flight_time() + sep)
        },
    };

    let valid = match &arr.window {
        None => true,
        Some(window) => window.as_range().contains(&landing),
    };

    valid
        .then_some(ArrivalSchedule {
            flight_index,
            landing,
        })
        .into_iter()
}

fn expand_departure(
    dep: &Departure,
    flight_index: usize,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = DepartureSchedule> {
    match &dep.deice {
        None => {
            let scheds = expand_direct_departure(dep, flight_index, instance, state);
            Left(scheds)
        },
        Some(deice) => {
            let scheds = expand_deiced_departure(dep, flight_index, deice, instance, state);
            Right(scheds)
        },
    }
}

fn expand_direct_departure(
    dep: &Departure,
    dep_idx: usize,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = DepartureSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);

    let takeoff = match prev_sched {
        None => dep.release_time(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];
            let takeoff = dep.release_time().max(prev_sched.flight_time() + sep);
            takeoff
        },
    };

    within_window(takeoff, dep.window.as_ref())
        .then_some(DepartureSchedule {
            flight_index: dep_idx,
            deice: None,
            takeoff,
        })
        .into_iter()
}

fn expand_deiced_departure(
    dep: &Departure,
    dep_idx: usize,
    deice: &Deice,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = DepartureSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);

    // Find the aircraft that deices last in the current solution
    let prev_deiced_sched = state
        .current_solution
        .iter()
        .rev()
        .filter_map(|node| {
            let sched = node.sched.as_departure()?;
            let deice = sched.deice.as_ref()?;
            Some((sched, deice))
        })
        .max_by_key(|(sched, &deice)| {
            let deice_dur = instance.flights()[sched.flight_index]
                .as_departure()
                .unwrap()
                .deice
                .as_ref()
                .unwrap()
                .duration;
            deice + deice_dur
        })
        .map(|(sched, _)| sched);

    let (earliest_deice, latest_deice, takeoff) = match (prev_sched, prev_deiced_sched) {
        (None, None) => {
            let takeoff = dep.release_time();

            let earliest_deice = (takeoff
                - instance.max_runway_hold_duration
                - dep.lineup_duration
                - dep.taxi_duration
                - deice.duration)
                .max(takeoff - deice.hot - deice.duration);
            let latest_deice = takeoff - dep.lineup_duration - dep.taxi_duration - deice.duration;

            (earliest_deice, latest_deice, takeoff)
        },
        (Some(prev_sched), None) => {
            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];

            let takeoff = dep.release_time().max(prev_sched.flight_time() + sep);

            let earliest_deice = (takeoff
                - instance.max_runway_hold_duration
                - dep.lineup_duration
                - dep.taxi_duration
                - deice.duration)
                .max(takeoff - deice.hot - deice.duration);
            let latest_deice = takeoff - dep.lineup_duration - dep.taxi_duration - deice.duration;

            (earliest_deice, latest_deice, takeoff)
        },
        (Some(prev_sched), Some(prev_deiced_sched)) => {
            let prev_deiced_dep = instance.flights()[prev_deiced_sched.flight_index]
                .as_departure()
                .unwrap();
            let prev_deice_end =
                prev_deiced_sched.deice.unwrap() + prev_deiced_dep.deice.as_ref().unwrap().duration;

            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];

            let takeoff =
                (prev_deice_end + deice.duration + dep.taxi_duration + dep.lineup_duration)
                    .max(dep.release_time())
                    .max(prev_sched.flight_time() + sep);

            let earliest_deice = (takeoff
                - instance.max_runway_hold_duration
                - dep.lineup_duration
                - dep.taxi_duration
                - deice.duration)
                .max(takeoff - deice.hot - deice.duration)
                .max(prev_deice_end);
            let latest_deice = takeoff - dep.lineup_duration - dep.taxi_duration - deice.duration;

            (earliest_deice, latest_deice, takeoff)
        },
        // PANICS: This is fine, because there can never be a case where there is no immediately
        //         preceding flight but there is a preceding departure.
        (None, Some(_)) => unreachable!(),
    };

    within_window(takeoff, dep.window.as_ref())
        .then_some(iter_minutes(earliest_deice, latest_deice))
        .into_iter()
        .flatten()
        .map(move |deice| DepartureSchedule {
            flight_index: dep_idx,
            deice: Some(deice),
            takeoff,
        })
}
