use std::{num::NonZeroUsize, ops::Range, time::Duration};

use chrono::NaiveTime;

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
        let mut sep_identical_sets = separation_identical_sets(instance);

        let horizon = match self.horizon {
            None => 0..instance.flights().len(),
            Some(horizon) => 0..usize::from(horizon),
        };

        let state = State {
            last_set_idxs: vec![0; sep_identical_sets.len()],
            current_schedule: Vec::new(),
            best_schedule: Vec::new(),
            current_bound: 0,
            lowest_bound: u64::MAX,
            horizon,
        };

        state.explore(instance, &mut sep_identical_sets)
    }
}

macro_rules! iter_minutes {
    ($range:expr, |$elem:pat_param| $body:block) => {
        {
            let minute = Duration::from_secs(60);
            let mut range = $range;
            while range.start <= range.end {
                let $elem = range.start;

                $body

                range.start += minute;
            }
        }
    };
}

struct State {
    last_set_idxs: Vec<usize>,
    current_schedule: Vec<Schedule>,
    best_schedule: Vec<Schedule>,
    current_bound: u64,
    lowest_bound: u64,
    horizon: Range<usize>,
}

impl State {
    fn explore(
        mut self,
        instance: &Instance,
        sep_identical_sets: &mut [Vec<usize>],
    ) -> Vec<Schedule> {
        self.explore_current(instance, sep_identical_sets);

        while self.horizon.end < instance.flights().len() {
            self.last_set_idxs.fill(0);

            self.best_schedule.drain(1..);
            let sched = self.best_schedule.pop().unwrap();

            sep_identical_sets
                .iter_mut()
                .for_each(|set| set.retain(|&flight_idx| flight_idx != sched.flight_index()));

            self.current_schedule.push(sched);

            self.horizon.start += 1;
            self.horizon.end += 1;

            self.explore_current(instance, sep_identical_sets);
        }

        self.current_schedule.extend(self.best_schedule);
        self.current_schedule
    }

    fn explore_current(&mut self, instance: &Instance, sep_identical_sets: &[Vec<usize>]) {
        if self.current_schedule.len() == self.horizon.end {
            self.update_best();
        } else {
            for (set_idx, sep_identical_set) in sep_identical_sets.iter().enumerate() {
                let last_set_idx = self.last_set_idxs[set_idx];
                let Some(&flight_idx) = sep_identical_set.get(last_set_idx) else {
                    continue;
                };

                let flight = &instance.flights()[flight_idx];
                match flight {
                    Flight::Arr(arrival) => self.explore_arrival(
                        arrival,
                        flight_idx,
                        instance,
                        sep_identical_sets,
                        set_idx,
                    ),
                    Flight::Dep(departure) => self.explore_departure(
                        departure,
                        flight_idx,
                        instance,
                        sep_identical_sets,
                        set_idx,
                    ),
                }
            }
        }
    }

    fn update_best(&mut self) {
        if self.current_bound < self.lowest_bound {
            self.lowest_bound = self.current_bound;
            self.best_schedule = self.current_schedule.clone();
        }
    }

    fn explore_next(
        &mut self,
        sched: Schedule,
        sched_cost: u64,
        instance: &Instance,
        sep_identical_sets: &[Vec<usize>],
        set_idx: usize,
    ) {
        self.current_schedule.push(sched);
        self.current_bound += sched_cost;
        self.last_set_idxs[set_idx] += 1;

        self.explore_current(instance, sep_identical_sets);

        self.current_schedule.pop();
        self.current_bound -= sched_cost;
        self.last_set_idxs[set_idx] -= 1;
    }

    fn explore_arrival(
        &mut self,
        arrival: &Arrival,
        arrival_idx: usize,
        instance: &Instance,
        sep_identical_sets: &[Vec<usize>],
        set_idx: usize,
    ) {
        let prev_sched = self.current_schedule.last();
        let landing_times = possible_landing_times(arrival, arrival_idx, prev_sched, instance);

        iter_minutes!(landing_times, |landing| {
            let sched = ArrivalSchedule {
                flight_idx: arrival_idx,
                landing,
            };

            let landing_cost = landing_cost(landing, arrival);
            if self.current_bound + landing_cost > self.lowest_bound {
                continue;
            }

            self.explore_next(
                sched.into(),
                landing_cost,
                instance,
                sep_identical_sets,
                set_idx,
            );
        });
    }

    fn explore_departure(
        &mut self,
        departure: &Departure,
        departure_idx: usize,
        instance: &Instance,
        sep_identical_sets: &[Vec<usize>],
        set_idx: usize,
    ) {
        let prev_sched = self.current_schedule.last();
        let prev_dep_sched = self
            .current_schedule
            .iter()
            .rev()
            .find_map(Schedule::as_departure)
            .cloned();

        let takeoff_times = possible_takeoff_times(departure, departure_idx, prev_sched, instance);

        iter_minutes!(takeoff_times, |takeoff| {
            let takeoff_cost = takeoff_cost(takeoff, departure);
            if self.current_bound + takeoff_cost > self.lowest_bound {
                continue;
            }

            let deice_times =
                possible_deice_times(departure, takeoff, prev_dep_sched.as_ref(), instance);

            iter_minutes!(deice_times, |deice| {
                let sched = DepartureSchedule {
                    flight_idx: departure_idx,
                    deice,
                    takeoff,
                };

                let slack_cost = slack_cost(deice, takeoff, departure);
                if self.current_bound + takeoff_cost + slack_cost > self.lowest_bound {
                    continue;
                }

                self.explore_next(
                    sched.into(),
                    takeoff_cost + slack_cost,
                    instance,
                    sep_identical_sets,
                    set_idx,
                );
            });
        });
    }
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

fn possible_landing_times(
    arrival: &Arrival,
    arrival_idx: usize,
    prev_sched: Option<&Schedule>,
    instance: &Instance,
) -> Range<NaiveTime> {
    match prev_sched {
        None => arrival.window.earliest()..arrival.window.latest(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), arrival_idx)];
            let earliest_landing = arrival
                .window
                .earliest()
                .max(prev_sched.flight_time() + sep);

            earliest_landing..arrival.window.latest()
        },
    }
}

fn possible_takeoff_times(
    departure: &Departure,
    departure_idx: usize,
    prev_sched: Option<&Schedule>,
    instance: &Instance,
) -> Range<NaiveTime> {
    match prev_sched {
        None => departure.ctot.earliest()..departure.ctot.latest(),
        Some(prev_sched) => {
            let sep = instance.separations()[(prev_sched.flight_index(), departure_idx)];
            let earliest_takeoff = departure
                .ctot
                .earliest()
                .max(prev_sched.flight_time() + sep);

            earliest_takeoff..departure.ctot.latest()
        },
    }
}

fn possible_deice_times(
    departure: &Departure,
    takeoff: NaiveTime,
    prev_dep_sched: Option<&DepartureSchedule>,
    instance: &Instance,
) -> Range<NaiveTime> {
    match prev_dep_sched {
        None => {
            let latest_deice =
                takeoff - departure.lineup_dur - departure.taxi_out_dur - departure.deice_dur;
            let earliest_deice = latest_deice - instance.max_slack_dur;

            earliest_deice..latest_deice
        },
        Some(prev_dep_sched) => {
            let prev_dep = instance.flights()[prev_dep_sched.flight_idx]
                .as_departure()
                .unwrap();
            let prev_deice_finish = prev_dep_sched.deice + prev_dep.deice_dur;

            let latest_deice =
                takeoff - departure.lineup_dur - departure.taxi_out_dur - departure.deice_dur;
            let earliest_deice = prev_deice_finish.max(latest_deice - instance.max_slack_dur);

            earliest_deice..latest_deice
        },
    }
}

fn landing_cost(landing: NaiveTime, arrival: &Arrival) -> u64 {
    let delay = landing - arrival.window.target;
    delay.num_minutes().unsigned_abs().pow(2)
}

fn takeoff_cost(takeoff: NaiveTime, departure: &Departure) -> u64 {
    let delay = takeoff - departure.ctot.target;
    delay.num_minutes().unsigned_abs().pow(2)
}

fn slack_cost(deice: NaiveTime, takeoff: NaiveTime, departure: &Departure) -> u64 {
    let tightest_deice =
        takeoff - departure.lineup_dur - departure.taxi_out_dur - departure.deice_dur;
    let slack = tightest_deice - deice;
    slack.num_minutes().unsigned_abs().pow(2)
}
