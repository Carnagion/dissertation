use std::{iter, num::NonZeroUsize, ops::Range, time::Duration};

use chrono::NaiveTime;

use either::{Left, Right};

use irdis_instance::{
    flight::{Arrival, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::{
    complete_orders::separation_identical_sets,
    cost::{arrival_cost, departure_cost, Cost},
};

#[derive(Debug, Clone)]
struct Node {
    sched: Schedule,
    depth: usize,
    sep_set_idx: usize,
    cost: u64,
}

pub fn branch_and_bound(
    instance: &Instance,
    horizon: Option<NonZeroUsize>,
) -> Option<Vec<Schedule>> {
    let mut sep_sets = separation_identical_sets(instance);
    println!("{:#?}", sep_sets);

    let mut next_in_sep_sets = vec![0; sep_sets.len()];

    let mut current_solution = Vec::with_capacity(instance.flights().len());
    let mut best_solution = current_solution.clone();

    let end = horizon
        .map(usize::from)
        .unwrap_or(instance.flights().len())
        .min(instance.flights().len());

    let mut nodes = Vec::with_capacity(instance.flights().len());

    branch_and_bound_once(
        instance,
        &sep_sets,
        &mut next_in_sep_sets,
        &mut current_solution,
        &mut best_solution,
        &mut nodes,
        0..end,
    );

    let windows = (1..)
        .zip(end + 1..=instance.flights().len())
        .map(|(start, end)| start..end);
    for window in windows {
        let fixed = best_solution.drain(..).next()?;
        let fixed_idx = fixed.sched.flight_index();
        current_solution.push(fixed);

        next_in_sep_sets.fill(0);
        for sep_set in &mut sep_sets {
            sep_set.retain(|&flight_idx| flight_idx != fixed_idx);
        }

        branch_and_bound_once(
            instance,
            &sep_sets,
            &mut next_in_sep_sets,
            &mut current_solution,
            &mut best_solution,
            &mut nodes,
            window.clone(),
        );
    }

    let solution = current_solution
        .into_iter()
        .chain(best_solution)
        .map(|node| node.sched)
        .collect();
    Some(solution)
}

fn branch_and_bound_once(
    instance: &Instance,
    sep_sets: &[Vec<usize>],
    next_in_sep_sets: &mut [usize],
    current_solution: &mut Vec<Node>,
    best_solution: &mut Vec<Node>,
    nodes: &mut Vec<Node>,
    horizon: Range<usize>,
) {
    let mut cost = Cost::default();

    nodes.extend(generate_next_nodes(
        instance,
        sep_sets,
        next_in_sep_sets,
        current_solution,
        horizon.start,
    ));

    while let Some(node) = nodes.pop() {
        let depth = node.depth;

        for node in current_solution.drain(depth..) {
            next_in_sep_sets[node.sep_set_idx] -= 1;
            cost.current -= node.cost;
        }

        if cost.current + node.cost >= cost.lowest {
            continue;
        }

        cost.current += node.cost;
        next_in_sep_sets[node.sep_set_idx] += 1;
        current_solution.push(node);

        if current_solution.len() == horizon.end {
            cost.lowest = cost.current;
            *best_solution = current_solution[horizon.clone()].to_vec();
            continue;
        }

        nodes.extend(generate_next_nodes(
            instance,
            sep_sets,
            next_in_sep_sets,
            current_solution,
            depth + 1,
        ));
    }

    current_solution.drain(horizon.start..);
}

fn generate_next_nodes<'a>(
    instance: &'a Instance,
    sep_sets: &[Vec<usize>],
    next_in_sep_sets: &[usize],
    current_solution: &'a [Node],
    depth: usize,
) -> impl DoubleEndedIterator<Item = Node> + 'a {
    let prev_earliest = current_solution.last().map(|node| {
        let flight_idx = node.sched.flight_index();
        let flight = &instance.flights()[flight_idx];
        flight.time_window().earliest
    });

    let mut next_flights = sep_sets
        .iter()
        .enumerate()
        .filter_map(|(sep_set_idx, sep_set)| {
            let next_idx = next_in_sep_sets[sep_set_idx];

            let flight_idx = sep_set.get(next_idx).copied()?;
            let flight = &instance.flights()[flight_idx];

            // NOTE: Prunes nodes according to complete orders induced by disjoint time windows.
            //
            //       For example, let the last scheduled flight in the current solution be `i`. If
            //       a candidate `j` flight to be scheduled next has a time window that is disjoint
            //       with `i`'s  and the latest time of `j` is before the earliest time of `i`, then
            //       scheduling `j` after `i` would lead to an infeasible solution. Thus, any solution
            //       containing `j` after `i` can be pruned.
            match prev_earliest {
                None => Some((flight, flight_idx, sep_set_idx)),
                Some(prev_earliest) if flight.time_window().latest <= prev_earliest => None,
                Some(_) => Some((flight, flight_idx, sep_set_idx)),
            }
        })
        .collect::<Vec<_>>();
    next_flights.sort_unstable_by_key(|(flight, ..)| flight.release_time());

    // NOTE: Prunes nodes according to complete orders induced by disjoint time windows.
    //
    //       For example, let the earliest flight among the set of candidate flights to be scheduled
    //       next be `i`. If there is any candidate flight `j` such that `j`'s time window is disjoint
    //       with `i`'s and the earliest time of `j` is after the latest time of `i`, then any feasible
    //       solution will always schedule `j` after `i`. Thus, `j` can be pruned from any candidate set
    //       that contains `i`.
    let next_latest = next_flights
        .iter()
        .map(|(flight, ..)| flight.time_window().latest)
        .min();
    if let Some(next_latest) = next_latest {
        next_flights.retain(|(flight, ..)| flight.time_window().earliest < next_latest);
    }

    next_flights
        .into_iter()
        .rev() // NOTE: Since the last added node is explored first, the best node should be added last.
               //       Here, the first node has the earliest time in the take-off or landing window, and is thus
               //       potentially the best and should be explored first. Reversing the iterator ensures this.
        .flat_map(move |(flight, flight_idx, sep_set_idx)| {
            generate_schedules(flight, flight_idx, current_solution, instance).map(
                move |(sched, cost)| Node {
                    sched,
                    cost,
                    depth,
                    sep_set_idx,
                },
            )
        })
}

fn generate_schedules<'a>(
    flight: &'a Flight,
    flight_idx: usize,
    current_solution: &[Node],
    instance: &Instance,
) -> impl DoubleEndedIterator<Item = (Schedule, u64)> + 'a {
    match flight {
        Flight::Arr(arr) => {
            let scheds = generate_arrivals(arr, flight_idx, current_solution, instance);
            Left(scheds.map(|sched| {
                let cost = arrival_cost(&sched, arr);
                (Schedule::Arr(sched), cost)
            }))
        },
        Flight::Dep(dep) => {
            let scheds = generate_departures(dep, flight_idx, current_solution, instance);
            Right(scheds.map(|sched| {
                let cost = departure_cost(&sched, dep);
                (Schedule::Dep(sched), cost)
            }))
        },
    }
}

fn generate_arrivals(
    arr: &Arrival,
    arr_idx: usize,
    current_solution: &[Node],
    instance: &Instance,
) -> impl DoubleEndedIterator<Item = ArrivalSchedule> {
    let prev_sched = current_solution.last().map(|node| &node.sched);

    let landing = match prev_sched {
        None => arr.release_time(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), arr_idx)];
            arr.release_time().max(prev_sched.flight_time() + sep)
        },
    };

    // TODO: Filter out solutions where the landing is outside the window
    iter::once(landing).map(move |landing| ArrivalSchedule {
        flight_idx: arr_idx,
        landing,
    })
}

fn generate_departures(
    dep: &Departure,
    dep_idx: usize,
    current_solution: &[Node],
    instance: &Instance,
) -> impl DoubleEndedIterator<Item = DepartureSchedule> {
    let prev_sched = current_solution.last().map(|node| &node.sched);
    let prev_dep_sched = current_solution
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
        (None, Some(_)) => unreachable!(),
    };

    // TODO: Filter out solutions where the takeoff is outside the window
    iter_minutes(earliest_deice, latest_deice)
        .rev() // NOTE: Since the last added node is explored first, the best node should be added last.
               //       Here, the first node leaves the least gaps in the de-icing queue, and is thus
               //       potentially the best and should be explored first. Reversing the iterator ensures this.
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
