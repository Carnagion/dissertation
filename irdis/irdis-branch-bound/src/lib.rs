use std::{num::NonZeroUsize, ops::Range, time::Duration};

use irdis_core::{
    instance::{
        op::{ArrivalConstraints, DepartureConstraints, OpConstraints},
        schedule::{ArrivalSchedule, DepartureSchedule, Op, OpSchedule, RunwaySchedule},
        Instance,
        SeparationSets,
    },
    solve::Solve,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BranchBound {
    pub horizon: Option<NonZeroUsize>,
}

impl BranchBound {
    pub fn new() -> Self {
        Self { horizon: None }
    }

    pub fn with_rolling_horizon<N>(horizon: N) -> Self
    where
        N: Into<Option<NonZeroUsize>>,
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
    fn solve(&self, instance: &Instance) -> RunwaySchedule {
        let mut separation_sets = instance.separation_sets();
        let mut last_set_indices = vec![0; separation_sets.0.len()];

        let mut schedule = RunwaySchedule(Vec::with_capacity(instance.rows().len()));
        let mut best_schedule = schedule.clone();

        let mut horizon = match self.horizon {
            None => 0..instance.rows().len(),
            Some(rolling_horizon) => 0..usize::from(rolling_horizon),
        };

        branch(
            &separation_sets,
            &mut last_set_indices,
            &mut schedule,
            instance,
            &mut Bounds::default(),
            &mut best_schedule,
            horizon.clone(),
        );

        while horizon.end < instance.rows().len() {
            last_set_indices.fill(0);

            best_schedule.0.drain(1..);
            let op = best_schedule.0.pop().unwrap();

            for separation_set in &mut separation_sets.0 {
                separation_set.retain(|&aircraft_idx| aircraft_idx != op.aircraft_idx);
            }

            schedule.0.push(op);

            horizon.start += 1;
            horizon.end += 1;

            branch(
                &separation_sets,
                &mut last_set_indices,
                &mut schedule,
                instance,
                &mut Bounds::default(),
                &mut best_schedule,
                horizon.clone(),
            );
        }

        schedule.0.extend(best_schedule.0);

        schedule
    }
}

#[derive(Debug)]
struct Bounds {
    lowest: f64,
    current: f64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            lowest: f64::INFINITY,
            current: 0.0,
        }
    }
}

fn branch(
    separation_sets: &SeparationSets,
    last_set_indices: &mut [usize],
    schedule: &mut RunwaySchedule,
    instance: &Instance,
    bounds: &mut Bounds,
    best_schedule: &mut RunwaySchedule,
    horizon: Range<usize>,
) {
    if schedule.0.len() == horizon.end {
        if bounds.current < bounds.lowest {
            bounds.lowest = bounds.current;
            *best_schedule = RunwaySchedule(schedule.0[horizon.clone()].to_vec());
        }

        return;
    }

    for (set_idx, separation_set) in separation_sets.0.iter().enumerate() {
        let last_set_idx = last_set_indices[set_idx];
        if last_set_idx >= separation_set.len() {
            continue;
        }

        let aircraft_idx = separation_set[last_set_idx];
        let op = schedule_aircraft(aircraft_idx, &schedule, instance);

        let current_cost = op_cost(&op, instance);
        let remaining_cost =
            estimated_remaining_bound(separation_sets, last_set_indices, instance, &op);
        let current_bound = bounds.current + current_cost + remaining_cost;

        if current_bound > bounds.lowest {
            continue;
        }

        schedule.0.push(op);
        bounds.current = current_bound;
        last_set_indices[set_idx] += 1;

        branch(
            separation_sets,
            last_set_indices,
            schedule,
            instance,
            bounds,
            best_schedule,
            horizon.clone(),
        );

        schedule.0.pop();
        bounds.current -= current_cost + remaining_cost;
        last_set_indices[set_idx] -= 1;
    }
}

fn bound<S>(schedule: S, instance: &Instance) -> f64
where
    S: IntoIterator<Item = Op>,
{
    schedule.into_iter().map(|op| op_cost(&op, instance)).sum()
}

fn op_cost(op: &Op, instance: &Instance) -> f64 {
    let earliest_time = instance.rows()[op.aircraft_idx].constraints.earliest_time();
    let actual_time = op.schedule.op_time();
    let diff = (actual_time - earliest_time).num_minutes() as f64;
    diff.powi(2)
}

fn estimated_remaining_bound(
    separation_sets: &SeparationSets,
    last_set_indices: &[usize],
    instance: &Instance,
    op: &Op,
) -> f64 {
    let assumed_separation = Duration::from_secs(60);

    separation_sets
        .0
        .iter()
        .zip(last_set_indices)
        .map(|(separation_set, &last_set_idx)| {
            separation_set[last_set_idx..]
                .iter()
                .scan(op.clone(), |prev_op, &aircraft_idx| {
                    let earliest_time = match &instance.rows()[aircraft_idx].constraints {
                        OpConstraints::Departure(constraints) => constraints.earliest_time,
                        OpConstraints::Arrival(constraints) => constraints.earliest_time,
                    };

                    let schedule = match &mut prev_op.schedule {
                        OpSchedule::Departure(prev_op) => {
                            prev_op.take_off_time =
                                earliest_time.max(prev_op.take_off_time + assumed_separation);
                            prev_op.de_ice_time += assumed_separation;
                            OpSchedule::Departure(DepartureSchedule {
                                take_off_time: prev_op.take_off_time,
                                de_ice_time: prev_op.de_ice_time,
                            })
                        },
                        OpSchedule::Arrival(prev_op) => {
                            prev_op.landing_time =
                                earliest_time.max(prev_op.landing_time + assumed_separation);
                            OpSchedule::Arrival(ArrivalSchedule {
                                landing_time: prev_op.landing_time,
                            })
                        },
                    };

                    Some(Op {
                        aircraft_idx,
                        schedule,
                    })
                })
        })
        .map(|schedule| bound(schedule, instance))
        .sum()
}

fn schedule_aircraft(aircraft_idx: usize, schedule: &RunwaySchedule, instance: &Instance) -> Op {
    let constraints = &instance.rows()[aircraft_idx].constraints;
    match constraints {
        OpConstraints::Departure(constraints) => {
            schedule_departure(aircraft_idx, constraints, schedule, instance)
        },
        OpConstraints::Arrival(constraints) => {
            schedule_arrival(aircraft_idx, constraints, schedule, instance)
        },
    }
}

fn schedule_departure(
    aircraft_idx: usize,
    constraints: &DepartureConstraints,
    schedule: &RunwaySchedule,
    instance: &Instance,
) -> Op {
    let prev_op = schedule.0.last();

    let earliest_take_off_time = match prev_op {
        None => constraints.earliest_time,
        Some(prev_op) => {
            let separation = instance
                .separation(prev_op.aircraft_idx, aircraft_idx)
                .unwrap();

            constraints
                .earliest_time
                .max(prev_op.schedule.op_time() + separation)
        },
    };

    let last_dep = schedule.0.iter().rev().find_map(|op| match &op.schedule {
        OpSchedule::Departure(last_dep) => Some((op.aircraft_idx, last_dep)),
        _ => None,
    });

    let earliest_de_ice_time = match last_dep {
        None => constraints.target_de_ice_time(),
        Some((last_dep_idx, last_dep)) => {
            let OpConstraints::Departure(last_dep_constraints) =
                &instance.rows()[last_dep_idx].constraints
            else {
                unreachable!()
            };

            (last_dep.de_ice_time + last_dep_constraints.de_ice_dur).max(
                earliest_take_off_time
                    - (constraints.lineup_dur
                        + constraints.post_de_ice_dur
                        + constraints.de_ice_dur),
            )
        },
    };

    let de_ice_time = earliest_de_ice_time;

    let take_off_time = earliest_take_off_time.max(
        de_ice_time + constraints.de_ice_dur + constraints.post_de_ice_dur + constraints.lineup_dur,
    );

    let schedule = OpSchedule::Departure(DepartureSchedule {
        take_off_time,
        de_ice_time,
    });

    Op {
        aircraft_idx,
        schedule,
    }
}

fn schedule_arrival(
    aircraft_idx: usize,
    constraints: &ArrivalConstraints,
    schedule: &RunwaySchedule,
    instance: &Instance,
) -> Op {
    let prev_op = schedule.0.last();

    let landing_time = match prev_op {
        None => constraints.earliest_time,
        Some(prev_op) => {
            let separation = instance
                .separation(prev_op.aircraft_idx, aircraft_idx)
                .unwrap();

            constraints
                .earliest_time
                .max(prev_op.schedule.op_time() + separation)
        },
    };

    let schedule = OpSchedule::Arrival(ArrivalSchedule { landing_time });

    Op {
        aircraft_idx,
        schedule,
    }
}
