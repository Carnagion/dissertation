use std::{num::NonZeroUsize, ops::Range};

use irdis_instance::{
    flight::{Arrival, Departure, Flight},
    schedule::{ArrivalSchedule, DepartureSchedule, Schedule},
    Instance,
    Solve,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BranchBound {
    pub horizon: Option<NonZeroUsize>,
}

impl BranchBound {
    pub fn new() -> Self {
        Self::with_rolling_horizon(None)
    }

    pub fn with_rolling_horizon<H>(horizon: H) -> Self
    where
        H: Into<Option<NonZeroUsize>>,
    {
        Self {
            horizon: horizon.into(),
        }
    }
}

impl Default for BranchBound {
    fn default() -> Self {
        Self::new()
    }
}

impl Solve for BranchBound {
    fn solve(&self, instance: &Instance) -> Option<Vec<Schedule>> {
        let mut sep_sets = separation_identical_sets(instance);
        let mut next_in_sep_sets = vec![0; sep_sets.len()];

        let mut current_solution = Vec::with_capacity(instance.flights().len());
        let mut best_solution = current_solution.clone();

        let mut horizon = match self.horizon {
            None => 0..instance.flights().len(),
            Some(horizon) => 0..usize::from(horizon).min(instance.flights().len()),
        };

        branch(
            instance,
            &sep_sets,
            &mut next_in_sep_sets,
            &mut current_solution,
            &mut best_solution,
            &mut Bounds::default(),
            horizon.clone(),
        );

        while horizon.end < instance.flights().len() {
            next_in_sep_sets.fill(0);

            let sched = best_solution.drain(..).next()?;

            for sep_set in &mut sep_sets {
                sep_set.retain(|&flight_idx| flight_idx != sched.flight_index());
            }

            current_solution.push(sched);

            horizon.start += 1;
            horizon.end += 1;

            branch(
                instance,
                &sep_sets,
                &mut next_in_sep_sets,
                &mut current_solution,
                &mut best_solution,
                &mut Bounds::default(),
                horizon.clone(),
            );
        }

        current_solution.extend(best_solution);
        Some(current_solution)
    }
}

fn branch(
    instance: &Instance,
    sep_sets: &[Vec<usize>],
    next_in_sep_sets: &mut [usize],
    current_solution: &mut Vec<Schedule>,
    best_solution: &mut Vec<Schedule>,
    bounds: &mut Bounds,
    horizon: Range<usize>,
) {
    if current_solution.len() == horizon.end {
        if bounds.current < bounds.lowest {
            bounds.lowest = bounds.current;
            *best_solution = current_solution[horizon.clone()].to_vec();
        }

        return;
    }

    for (set_idx, sep_set) in sep_sets.iter().enumerate() {
        let next_idx = next_in_sep_sets[set_idx];
        let Some(&flight_idx) = sep_set.get(next_idx) else {
            continue;
        };

        let flight = &instance.flights()[flight_idx];
        match flight {
            Flight::Arr(arr) => {
                for sched in possible_arrs(arr, flight_idx, current_solution, instance) {
                    let landing_cost = landing_cost(&sched, arr);
                    if bounds.current + landing_cost >= bounds.lowest {
                        continue;
                    }

                    current_solution.push(sched.into());
                    bounds.current += landing_cost;
                    next_in_sep_sets[set_idx] += 1;

                    branch(
                        instance,
                        sep_sets,
                        next_in_sep_sets,
                        current_solution,
                        best_solution,
                        bounds,
                        horizon.clone(),
                    );

                    current_solution.pop();
                    bounds.current -= landing_cost;
                    next_in_sep_sets[set_idx] -= 1;
                }
            },
            Flight::Dep(dep) => {
                for sched in possible_deps(dep, flight_idx, current_solution, instance) {
                    let takeoff_cost = takeoff_cost(&sched, dep);
                    let holdover_cost = holdover_cost(&sched, dep);
                    if bounds.current + takeoff_cost + holdover_cost >= bounds.lowest {
                        continue;
                    }

                    current_solution.push(sched.into());
                    bounds.current += takeoff_cost + holdover_cost;
                    next_in_sep_sets[set_idx] += 1;

                    branch(
                        instance,
                        sep_sets,
                        next_in_sep_sets,
                        current_solution,
                        best_solution,
                        bounds,
                        horizon.clone(),
                    );

                    current_solution.pop();
                    bounds.current -= takeoff_cost + holdover_cost;
                    next_in_sep_sets[set_idx] -= 1;
                }
            },
        }
    }
}

macro_rules! iter_minutes {
    ($from:expr, $to:expr) => {{
        let (from, to) = ($from, $to);
        let diff = (to - from).num_minutes().unsigned_abs();
        (0..diff + 1).map(move |minute| from + ::std::time::Duration::from_secs(minute * 60))
    }};
}

fn possible_arrs<'a>(
    arr: &'a Arrival,
    arr_idx: usize,
    current_solution: &[Schedule],
    instance: &Instance,
) -> impl Iterator<Item = ArrivalSchedule> + 'a {
    let prev_sched = current_solution.last();

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

    iter_minutes!(earliest_landing, latest_landing).map(move |landing| ArrivalSchedule {
        flight_idx: arr_idx,
        landing,
    })
}

fn possible_deps<'a>(
    dep: &'a Departure,
    dep_idx: usize,
    current_solution: &[Schedule],
    instance: &Instance,
) -> impl Iterator<Item = DepartureSchedule> + 'a {
    let prev_sched = current_solution.last();
    let prev_dep_sched = current_solution
        .iter()
        .rev()
        .find_map(Schedule::as_departure);

    let earliest_takeoff = match prev_sched {
        None => dep.ctot.earliest(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), dep_idx)];
            dep.ctot.earliest().max(prev_sched.flight_time() + sep)
        },
    };

    let (earliest_deice, latest_deice) = match prev_dep_sched {
        None => {
            let earliest_deice = earliest_takeoff - instance.max_holdover_dur - dep.deice_dur;
            let latest_deice = earliest_takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur;
            (earliest_deice, latest_deice)
        },
        Some(prev_dep_sched) => {
            let prev_dep = instance.flights()[prev_dep_sched.flight_idx]
                .as_departure()
                .unwrap();

            let prev_deice_finish = prev_dep_sched.deice + prev_dep.deice_dur;

            let earliest_deice = (earliest_takeoff
                - dep.lineup_dur
                - dep.taxi_out_dur
                - instance.max_slack_dur
                - dep.deice_dur)
                .max(earliest_takeoff - instance.max_holdover_dur - dep.deice_dur)
                .max(prev_deice_finish);
            let latest_deice =
                (earliest_takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur)
                    .max(prev_deice_finish);

            (earliest_deice, latest_deice)
        },
    };

    iter_minutes!(earliest_deice, latest_deice).map(move |deice| {
        let takeoff =
            earliest_takeoff.max(deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur);

        DepartureSchedule {
            flight_idx: dep_idx,
            deice,
            takeoff,
        }
    })
}

struct Bounds {
    current: u64,
    lowest: u64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            current: 0,
            lowest: u64::MAX,
        }
    }
}

fn landing_cost(sched: &ArrivalSchedule, arr: &Arrival) -> u64 {
    (sched.landing - arr.window.target)
        .num_minutes()
        .unsigned_abs()
        .pow(2)
}

fn takeoff_cost(sched: &DepartureSchedule, dep: &Departure) -> u64 {
    (sched.takeoff - dep.ctot.target)
        .num_minutes()
        .unsigned_abs()
        .pow(2)
}

fn holdover_cost(sched: &DepartureSchedule, dep: &Departure) -> u64 {
    let tightest_deice = sched.takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur;
    let slack = (tightest_deice - sched.deice).num_minutes().unsigned_abs();
    slack
}

fn separation_identical_sets(instance: &Instance) -> Vec<Vec<usize>> {
    let mut sets = Vec::<Vec<_>>::with_capacity(instance.flights().len().min(1));

    'unclassified: for unclassified in 0..instance.flights().len() {
        'sets: for set in &mut sets {
            for classified in set.iter().copied() {
                let other = (0..instance.flights().len())
                    .filter(|other| ![unclassified, classified].contains(other));
                for other in other {
                    let sep_unclassified = instance.separations().get(unclassified, other);
                    let sep_unclassified_rev = instance.separations().get(other, unclassified);

                    let sep_classified = instance.separations().get(classified, other);
                    let sep_classified_rev = instance.separations().get(other, classified);

                    if sep_unclassified != sep_classified
                        || sep_unclassified_rev != sep_classified_rev
                    {
                        continue 'sets;
                    }
                }
            }

            set.push(unclassified);
            continue 'unclassified;
        }

        let set = vec![unclassified];
        sets.push(set);
    }

    sets
}
