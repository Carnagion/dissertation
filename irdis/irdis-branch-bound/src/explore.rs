use std::{ops::Range, time::Duration};

use chrono::NaiveTime;

use irdis_instance::{
    flight::{Arrival, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
};

use crate::cost::{arrival_cost, departure_cost, Cost};

pub fn explore_sep_sets(
    instance: &Instance,
    sep_sets: &[Vec<usize>],
    next_in_sep_sets: &mut [usize],
    current_solution: &mut Vec<Schedule>,
    best_solution: &mut Vec<Schedule>,
    cost: &mut Cost,
    horizon: Range<usize>,
) {
    if current_solution.len() == horizon.end {
        update_best_solution(current_solution, best_solution, cost, horizon);
    } else {
        for (set_idx, sep_set) in sep_sets.iter().enumerate() {
            let next_idx = next_in_sep_sets[set_idx];
            let Some(&flight_idx) = sep_set.get(next_idx) else {
                continue;
            };

            let flight = &instance.flights()[flight_idx];
            match flight {
                Flight::Arr(arr) => {
                    for sched in possible_arrs(arr, flight_idx, current_solution, instance) {
                        let arr_cost = arrival_cost(&sched, arr);
                        if cost.current + arr_cost >= cost.best {
                            continue;
                        }

                        current_solution.push(sched.into());
                        cost.current += arr_cost;
                        next_in_sep_sets[set_idx] += 1;

                        explore_sep_sets(
                            instance,
                            sep_sets,
                            next_in_sep_sets,
                            current_solution,
                            best_solution,
                            cost,
                            horizon.clone(),
                        );

                        current_solution.pop();
                        cost.current -= arr_cost;
                        next_in_sep_sets[set_idx] -= 1;
                    }
                },
                Flight::Dep(dep) => {
                    for sched in possible_deps(dep, flight_idx, current_solution, instance) {
                        let dep_cost = departure_cost(&sched, dep);
                        if cost.current + dep_cost >= cost.best {
                            continue;
                        }

                        current_solution.push(sched.into());
                        cost.current += dep_cost;
                        next_in_sep_sets[set_idx] += 1;

                        explore_sep_sets(
                            instance,
                            sep_sets,
                            next_in_sep_sets,
                            current_solution,
                            best_solution,
                            cost,
                            horizon.clone(),
                        );

                        current_solution.pop();
                        cost.current -= dep_cost;
                        next_in_sep_sets[set_idx] -= 1;
                    }
                },
            }
        }
    }
}

fn update_best_solution(
    current_solution: &[Schedule],
    best_solution: &mut Vec<Schedule>,
    cost: &mut Cost,
    horizon: Range<usize>,
) {
    if cost.current < cost.best {
        cost.best = cost.current;
        *best_solution = current_solution[horizon].to_vec();
    }
}

fn possible_arrs(
    arr: &Arrival,
    arr_idx: usize,
    current_solution: &[Schedule],
    instance: &Instance,
) -> impl Iterator<Item = ArrivalSchedule> {
    let prev_sched = current_solution.last();

    // NOTE: Using the earliest landing as the latest landing effectively limits
    //       the operation to one time only - the earliest. This prunes the search space
    //       a lot but requires the objective function to consider `earliest()` and not
    //       the `target`.
    let (earliest_landing, latest_landing) = match prev_sched {
        None => (arr.window.earliest(), arr.window.earliest()),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), arr_idx)];
            let earliest_landing = arr.window.earliest().max(prev_sched.flight_time() + sep);
            // let latest_landing = arr.window.latest().max(earliest_landing);
            let latest_landing = earliest_landing;
            (earliest_landing, latest_landing)
        },
    };

    iter_minutes(earliest_landing, latest_landing).map(move |landing| ArrivalSchedule {
        flight_idx: arr_idx,
        landing,
    })
}

fn possible_deps(
    dep: &Departure,
    dep_idx: usize,
    current_solution: &[Schedule],
    instance: &Instance,
) -> impl Iterator<Item = DepartureSchedule> {
    let prev_sched = current_solution.last();
    let prev_dep_sched = current_solution
        .iter()
        .rev()
        .find_map(Schedule::as_departure)
        .cloned();

    let (earliest_deice, latest_deice, takeoff) = match (prev_sched, prev_dep_sched) {
        (None, None) => {
            let takeoff = dep.ctot.earliest();

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

            let takeoff = dep.ctot.earliest().max(prev_sched.flight_time() + sep);

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
                .max(dep.ctot.earliest())
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

    iter_minutes(earliest_deice, latest_deice).map(move |deice| DepartureSchedule {
        flight_idx: dep_idx,
        deice,
        takeoff,
    })
}

fn iter_minutes(from: NaiveTime, to: NaiveTime) -> impl Iterator<Item = NaiveTime> {
    let diff = (to - from)
        .max(chrono::Duration::zero())
        .num_minutes()
        .unsigned_abs();
    (0..=diff).map(move |minute| from + Duration::from_secs(minute * 60))
}
