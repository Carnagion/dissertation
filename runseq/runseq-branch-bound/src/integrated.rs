use std::num::NonZeroUsize;

use chrono::NaiveDateTime;
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

    // Perform branch-and-bound for the first window
    branch_bound(instance, &mut state, &mut nodes, &mut expand, 0..end);

    // Perform branch-and-bound for the remaining windows
    let windows = (1..)
        .zip(end + 1..=flight_count)
        .map(|(start, end)| start..end);
    for window in windows {
        // Ignore all scheduled aircraft except for the first one
        let fixed = state.best_solution.drain(..).next()?;
        let fixed_idx = fixed.sched.flight_index();

        // Save the first aircraft to the current solution
        state.current_solution.push(fixed);

        // Remove that aircraft from the sets of complete orders
        state.next_in_complete_order_sets.fill(0);
        for set in &mut state.complete_order_sets {
            set.retain(|&flight_idx| flight_idx != fixed_idx);
        }

        // Perform branch-and-bound for the current window
        branch_bound(instance, &mut state, &mut nodes, &mut expand, window);
    }

    // Ensure that a feasible last solution is produced
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
    // Find the time when all separation requirements with already scheduled aircraft are satisfied
    let sep_end = state
        .current_solution
        .iter()
        .rev()
        .map(|node| {
            let sep = instance.separations()[(node.sched.flight_index(), flight_index)];
            node.sched.flight_time() + sep
        })
        .max()
        .unwrap_or(NaiveDateTime::MIN);

    let landing = arr.release_time().max(sep_end);

    // Ensure that the scheduled landing time respects all constraints
    within_window(landing, arr.window.as_ref())
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
    flight_index: usize,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = DepartureSchedule> {
    // Find the time when all separation requirements with already scheduled aircraft are satisfied
    let sep_end = state
        .current_solution
        .iter()
        .rev()
        .map(|node| {
            let sep = instance.separations()[(node.sched.flight_index(), flight_index)];
            node.sched.flight_time() + sep
        })
        .max()
        .unwrap_or(NaiveDateTime::MIN);

    let takeoff = dep.release_time().max(sep_end);

    // Ensure that the scheduled take-off time respects all constraints
    within_window(takeoff, dep.window.as_ref())
        .then_some(DepartureSchedule {
            flight_index,
            deice: None,
            takeoff,
        })
        .into_iter()
}

fn expand_deiced_departure(
    dep: &Departure,
    flight_index: usize,
    deice: &Deice,
    instance: &Instance,
    state: &BranchBoundState,
) -> impl Iterator<Item = DepartureSchedule> {
    // Find the time when all separation requirements with already scheduled aircraft are satisfied
    let sep_end = state
        .current_solution
        .iter()
        .rev()
        .map(|node| {
            let sep = instance.separations()[(node.sched.flight_index(), flight_index)];
            node.sched.flight_time() + sep
        })
        .max()
        .unwrap_or(NaiveDateTime::MIN);

    // Find the time when the last de-icing aircraft finishes de-icing
    let deice_end = state
        .current_solution
        .iter()
        .rev()
        .filter_map(|node| {
            let sched = node.sched.as_departure()?;
            let deice_time = sched.deice?;
            let deice_dur = instance.flights()[sched.flight_index]
                .as_departure()?
                .deice
                .as_ref()?
                .duration;
            Some(deice_time + deice_dur)
        })
        .max();

    let (earliest_deice, latest_deice, takeoff) = match deice_end {
        None => {
            let takeoff = dep.release_time().max(sep_end);

            // If no aircraft was previously de-icing, then the current departure can be de-iced as soon as possible
            let earliest_deice = (takeoff
                - instance.max_runway_hold_duration
                - dep.lineup_duration
                - dep.taxi_duration
                - deice.duration)
                .max(takeoff - deice.hot - deice.duration);
            let latest_deice = takeoff - dep.lineup_duration - dep.taxi_duration - deice.duration;

            (earliest_deice, latest_deice, takeoff)
        },
        Some(deice_end) => {
            let takeoff = dep
                .release_time()
                .max(sep_end)
                .max(deice_end + deice.duration + dep.taxi_duration + dep.lineup_duration);

            // If there was an aircraft that was previously de-icing, then the current departure can only de-ice after
            // that one has finished
            let earliest_deice = (takeoff
                - instance.max_runway_hold_duration
                - dep.lineup_duration
                - dep.taxi_duration
                - deice.duration)
                .max(takeoff - deice.hot - deice.duration)
                .max(deice_end);
            let latest_deice = takeoff - dep.lineup_duration - dep.taxi_duration - deice.duration;

            (earliest_deice, latest_deice, takeoff)
        },
    };

    // Ensure that the scheduled take-off time and de-icing time respect all constraints
    within_window(takeoff, dep.window.as_ref())
        .then_some(iter_minutes(earliest_deice, latest_deice))
        .into_iter()
        .flatten()
        .map(move |deice| DepartureSchedule {
            flight_index,
            deice: Some(deice),
            takeoff,
        })
}
