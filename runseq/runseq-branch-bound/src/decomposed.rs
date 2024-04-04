use std::{cmp::Ordering, collections::HashMap, num::NonZeroUsize};

use chrono::NaiveDateTime;

use either::{Left, Right};

use runseq_instance::{
    flight::{Arrival, Deice, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::search::{branch_bound, within_window, BranchBoundState};

pub fn branch_bound_rolling<F>(
    instance: &Instance,
    horizon: Option<NonZeroUsize>,
    mut sorter: F,
) -> Option<Vec<Schedule>>
where
    F: FnMut(&Departure, &Departure) -> Ordering,
{
    let flight_count = instance.flights().len();

    let mut state = BranchBoundState::new(instance);

    let mut deice_queue = HashMap::new();
    generate_deice_queue(instance, &state, &mut deice_queue, &mut sorter);

    let end = horizon
        .map(usize::from)
        .unwrap_or(flight_count)
        .min(flight_count);

    let mut nodes = Vec::with_capacity(flight_count);

    branch_bound(
        instance,
        &mut state,
        &mut nodes,
        &mut |flight, flight_index, instance, state| {
            expand(flight, flight_index, instance, state, &deice_queue)
        },
        0..end,
    );

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

        generate_deice_queue(instance, &state, &mut deice_queue, &mut sorter);

        branch_bound(
            instance,
            &mut state,
            &mut nodes,
            &mut |flight, flight_index, instance, state| {
                expand(flight, flight_index, instance, state, &deice_queue)
            },
            window,
        );
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
    deice_queue: &HashMap<usize, NaiveDateTime>,
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
    deice_queue: &HashMap<usize, NaiveDateTime>,
) -> impl Iterator<Item = DepartureSchedule> {
    match &dep.deice {
        None => {
            let scheds = expand_direct_departure(dep, flight_index, instance, state);
            Left(scheds)
        },
        Some(deice) => {
            let scheds =
                expand_deiced_departure(dep, flight_index, deice, instance, state, deice_queue);
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
    flight_index: usize,
    deice_params: &Deice,
    instance: &Instance,
    state: &BranchBoundState,
    deice_queue: &HashMap<usize, NaiveDateTime>,
) -> impl Iterator<Item = DepartureSchedule> {
    let prev_sched = state.current_solution.last().map(|node| &node.sched);

    let (deice, takeoff) = match prev_sched {
        None => {
            let deice = deice_queue[&flight_index];
            let takeoff = (deice + deice_params.duration + dep.taxi_duration + dep.lineup_duration)
                .max(dep.release_time());

            (deice, takeoff)
        },
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), flight_index)];

            let deice = deice_queue[&flight_index];
            let takeoff = (deice + deice_params.duration + dep.taxi_duration + dep.lineup_duration)
                .max(dep.release_time())
                .max(prev_sched.flight_time() + sep);

            (deice, takeoff)
        },
    };

    let valid = within_window(takeoff, dep.window.as_ref())
        && takeoff <= deice + deice_params.duration + deice_params.hot
        && takeoff
            <= deice
                + deice_params.duration
                + dep.taxi_duration
                + instance.max_runway_hold_duration
                + dep.lineup_duration;

    valid
        .then_some(DepartureSchedule {
            flight_index,
            deice: Some(deice),
            takeoff,
        })
        .into_iter()
}

fn generate_deice_queue<F>(
    instance: &Instance,
    state: &BranchBoundState,
    deice_queue: &mut HashMap<usize, NaiveDateTime>,
    sorter: &mut F,
) where
    F: FnMut(&Departure, &Departure) -> Ordering,
{
    let mut remaining_departures = instance
        .flights()
        .iter()
        .enumerate()
        .filter_map(|(flight_idx, flight)| {
            let dep = flight.as_departure()?;
            let deice = dep.deice.as_ref()?;
            let not_scheduled = state
                .current_solution
                .iter()
                .all(|node| node.sched.flight_index() != flight_idx);
            not_scheduled.then_some((flight_idx, dep, deice))
        })
        .collect::<Vec<_>>();
    remaining_departures.sort_unstable_by(|(_, dep, _), (_, other, _)| sorter(dep, other));

    let last_deice_end = state
        .current_solution
        .iter()
        .rev()
        .filter_map(|node| node.sched.as_departure())
        .filter_map(|sched| match sched.deice {
            None => None,
            Some(deice) => {
                let dep = instance.flights()[sched.flight_index]
                    .as_departure()
                    .unwrap();
                Some(deice + dep.deice.as_ref().unwrap().duration)
            },
        })
        .max();

    let remaining_queue = remaining_departures.into_iter().scan(
        last_deice_end,
        |last_deice_end, (flight_idx, dep, deice)| {
            let mut deice =
                (dep.release_time() - dep.lineup_duration - dep.taxi_duration - deice.duration)
                    .max(dep.release_time() - deice.hot - deice.duration);
            if let Some(last_deice_end) = last_deice_end {
                deice = deice.max(*last_deice_end);
            }

            *last_deice_end = Some(deice);

            Some((flight_idx, deice))
        },
    );

    deice_queue.clear();
    deice_queue.extend(remaining_queue);
}
