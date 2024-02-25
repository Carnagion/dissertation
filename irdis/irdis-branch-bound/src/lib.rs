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
    fn solve(&self, instance: &Instance) -> Vec<Schedule> {
        let mut sep_sets = separation_identical_sets(instance);

        let horizon = match self.horizon {
            None => 0..instance.flights().len(),
            Some(horizon) => 0..usize::from(horizon),
        };

        let mut state = State {
            next_in_sep_sets: vec![0; sep_sets.len()],
            current_solution: Vec::with_capacity(instance.flights().len()),
            best_solution: Vec::new(),
            bounds: Bounds::default(),
            horizon,
        };

        state.branch_sep_sets(instance, &sep_sets);

        while state.horizon.end < instance.flights().len() {
            state.next_in_sep_sets.fill(0);

            state.best_solution.drain(1..);
            let sched = state.best_solution.pop().unwrap();

            sep_sets
                .iter_mut()
                .for_each(|set| set.retain(|&flight_idx| flight_idx != sched.flight_index()));

            state.current_solution.push(sched);

            state.bounds = Bounds::default();

            state.horizon.start += 1;
            state.horizon.end += 1;

            state.branch_sep_sets(instance, &sep_sets);
        }

        state.current_solution.extend(state.best_solution);
        state.current_solution
    }
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

struct State {
    next_in_sep_sets: Vec<usize>,
    current_solution: Vec<Schedule>,
    best_solution: Vec<Schedule>,
    bounds: Bounds,
    horizon: Range<usize>,
}

impl State {
    fn branch_sep_sets(&mut self, instance: &Instance, sep_sets: &[Vec<usize>]) {
        if self.current_solution.len() == self.horizon.end {
            self.update_best_solution();
        } else {
            for (set_idx, sep_set) in sep_sets.iter().enumerate() {
                let next_idx = self.next_in_sep_sets[set_idx];
                let Some(&flight_idx) = sep_set.get(next_idx) else {
                    continue;
                };
                self.branch_flight(flight_idx, set_idx, instance, sep_sets);
            }
        }
    }

    fn branch_flight(
        &mut self,
        flight_idx: usize,
        set_idx: usize,
        instance: &Instance,
        sep_sets: &[Vec<usize>],
    ) {
        let flight = &instance.flights()[flight_idx];
        match flight {
            Flight::Arr(arr) => {
                for sched in possible_arrs(arr, flight_idx, &self.current_solution, instance) {
                    let landing_cost = landing_cost(&sched, arr);
                    if self.bounds.current + landing_cost > self.bounds.lowest {
                        continue;
                    }
                    self.branch_next(sched, landing_cost, set_idx, instance, sep_sets);
                }
            },
            Flight::Dep(dep) => {
                for sched in possible_deps(dep, flight_idx, &self.current_solution, instance) {
                    let holdover_cost = holdover_cost(&sched, dep);
                    if self.bounds.current + holdover_cost > self.bounds.lowest {
                        continue;
                    }
                    self.branch_next(sched, holdover_cost, set_idx, instance, sep_sets);
                }
            },
        }
    }

    fn branch_next<S>(
        &mut self,
        sched: S,
        cost: u64,
        set_idx: usize,
        instance: &Instance,
        sep_sets: &[Vec<usize>],
    ) where
        S: Into<Schedule>,
    {
        let sched = sched.into();

        self.current_solution.push(sched);
        self.bounds.current += cost;
        self.next_in_sep_sets[set_idx] += 1;

        self.branch_sep_sets(instance, sep_sets);

        self.current_solution.pop();
        self.bounds.current -= cost;
        self.next_in_sep_sets[set_idx] -= 1;
    }

    fn update_best_solution(&mut self) {
        if self.bounds.current < self.bounds.lowest {
            self.bounds.lowest = self.bounds.current;
            self.best_solution = self.current_solution.clone();
        }
    }
}

// NOTE: This is a workaround the fact that `Range<T>` currently only impls `Iterator` if
//       `T` impls `Step`. Of course, `Step` is still an unstable trait, so `chrono::NaiveTime`
//       does not impl it, and thus it isn't possible to iterate over a `Range<NaiveTime>`.
macro_rules! iter_minutes {
    ($from:expr, $to:expr) => {{
        let (from, to) = ($from, $to);
        let diff = (from - to).num_minutes().unsigned_abs();
        (0..=diff).map(move |minute| from + ::std::time::Duration::from_secs(minute * 60))
    }};
}

fn possible_arrs(
    arr: &Arrival,
    arr_idx: usize,
    current_solution: &[Schedule],
    instance: &Instance,
) -> impl Iterator<Item = ArrivalSchedule> {
    let prev_sched = current_solution.last();

    let (earliest_landing, latest_landing) = match prev_sched {
        None => (arr.window.earliest(), arr.window.latest()),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), arr_idx)];
            let earliest_landing = arr.window.earliest().max(prev_sched.flight_time() + sep);
            (earliest_landing, arr.window.latest())
        },
    };

    iter_minutes!(earliest_landing, latest_landing).map(move |landing| ArrivalSchedule {
        flight_idx: arr_idx,
        landing,
    })
}

// NOTE: Since there is no penalty for taking off anywhere inside a departure's assigned
//       CTOT slot, we try to take off as early as we possibly can.
//
//       First, the earliest possible take-off time is calculated based on the earliest
//       allowed time in the departure's CTOT slot and the take-off or landing time of the
//       previous flight plus separation, if any.
//
//       The departure's earliest and latest de-icing times can be calculated based on this.
//
//       If this is the first departure in the sequence, its earliest de-icing time is its
//       earliest take-off time minus the maximum holdover duration minus its de-icing duration.
//       Its latest de-icing time is its earliest take-off time minus its taxi and lineup durations
//       minus its de-icing duration.
//
//       If there is a departure before it, its earliest and latest de-icing times are calculated
//       in a similar manner as above, except are also compared with the time that the previous
//       departure finishes de-icing, and the maximum of the values is taken.
//
//       On one hand, we want to de-ice as soon as possible to avoid any gaps in the de-icing queue,
//       as these have a significantly large knock-on effect on the following departures. On the other
//       hand, this means that delays are absorbed at the runway rather than at the gates, wasting fuel.
//       The ideal de-icing time lies somewhere in this range, and depends on the delay and slack costs.
//
//       Finally, for each de-ice time within the range, we can calculate the corresponding take-off
//       time as the maximum of the earliest takeoff time (calculated previously) and the time it
//       takes to finish de-icing and get to the runway.
//
//       Since the earliest possible take-off time will always be greater than or equal to the CTOT
//       window's earliest allowed time, and the actual take-off time will always be greater than
//       or equal to the earliest take-off time, we know for sure that the take-off time will be
//       after the CTOT window's earliest allowed time. However, it may still be after its latest
//       allowed time, in which case the current branch becomes infeasible and can be ignored.
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
        .find_map(Schedule::as_departure)
        .cloned();

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

            let earliest_deice = (earliest_takeoff - instance.max_holdover_dur - dep.deice_dur)
                .max(prev_dep_sched.deice + prev_dep.deice_dur);
            let latest_deice =
                (earliest_takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur)
                    .max(prev_dep_sched.deice + prev_dep.deice_dur);

            (earliest_deice, latest_deice)
        },
    };

    iter_minutes!(earliest_deice, latest_deice).filter_map(move |deice| {
        let takeoff =
            earliest_takeoff.max(deice + dep.deice_dur + dep.taxi_out_dur + dep.lineup_dur);

        (dep.ctot.earliest()..=dep.ctot.latest())
            .contains(&takeoff)
            .then_some(DepartureSchedule {
                flight_idx: dep_idx,
                deice,
                takeoff,
            })
    })
}

fn landing_cost(sched: &ArrivalSchedule, arr: &Arrival) -> u64 {
    let delay = sched.landing - arr.window.target;
    delay.num_minutes().unsigned_abs().pow(2)
}

fn holdover_cost(sched: &DepartureSchedule, dep: &Departure) -> u64 {
    let tightest_deice = sched.takeoff - dep.lineup_dur - dep.taxi_out_dur - dep.deice_dur;
    let slack = tightest_deice - sched.deice;
    slack.num_minutes().unsigned_abs().pow(2)
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
