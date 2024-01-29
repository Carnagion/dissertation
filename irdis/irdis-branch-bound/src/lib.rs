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
pub struct BranchBound {}

impl Default for BranchBound {
    fn default() -> Self {
        Self {}
    }
}

impl Solve for BranchBound {
    fn solve(&self, instance: &Instance) -> RunwaySchedule {
        let separation_sets = instance.separation_sets();
        let mut last_set_indices = vec![0; separation_sets.0.len()];

        let mut schedules = RunwaySchedule(Vec::with_capacity(instance.rows().len()));
        let mut best_schedules = schedules.clone();

        branch(
            &separation_sets,
            &mut last_set_indices,
            &mut schedules,
            instance,
            &mut Bounds::default(),
            &mut best_schedules,
        );

        best_schedules
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
    schedules: &mut RunwaySchedule,
    instance: &Instance,
    bounds: &mut Bounds,
    best_schedules: &mut RunwaySchedule,
) {
    if schedules.0.len() == instance.rows().len() {
        if bounds.current < bounds.lowest {
            bounds.lowest = bounds.current;
            *best_schedules = schedules.clone();
        }

        return;
    }

    for (set_idx, separation_set) in separation_sets.0.iter().enumerate() {
        let last_set_idx = last_set_indices[set_idx];
        if last_set_idx >= separation_set.len() {
            continue;
        }

        let aircraft_idx = separation_set[last_set_idx];
        let op = schedule_aircraft(aircraft_idx, &schedules, instance);

        let current_cost = op_cost(&op, instance);
        let current_bound = bounds.current + current_cost;

        if current_bound > bounds.lowest {
            continue;
        }

        schedules.0.push(op);
        bounds.current = current_bound;
        last_set_indices[set_idx] += 1;

        branch(
            separation_sets,
            last_set_indices,
            schedules,
            instance,
            bounds,
            best_schedules,
        );

        schedules.0.pop();
        bounds.current -= current_cost;
        last_set_indices[set_idx] -= 1;
    }
}

fn bound(schedule: &RunwaySchedule, instance: &Instance) -> f64 {
    schedule.0.iter().map(|op| op_cost(op, instance)).sum()
}

fn op_cost(op: &Op, instance: &Instance) -> f64 {
    let earliest_time = instance.rows()[op.aircraft_idx].constraints.earliest_time();
    let actual_time = op.schedule.op_time();
    let diff = (actual_time - earliest_time).num_minutes() as f64;
    diff.powi(2)
}

fn schedule_aircraft(aircraft_idx: usize, schedules: &RunwaySchedule, instance: &Instance) -> Op {
    let constraints = &instance.rows()[aircraft_idx].constraints;
    match constraints {
        OpConstraints::Departure(constraints) => {
            schedule_departure(aircraft_idx, constraints, schedules, instance)
        },
        OpConstraints::Arrival(constraints) => {
            schedule_arrival(aircraft_idx, constraints, schedules, instance)
        },
    }
}

fn schedule_departure(
    aircraft_idx: usize,
    constraints: &DepartureConstraints,
    schedules: &RunwaySchedule,
    instance: &Instance,
) -> Op {
    let prev_op = schedules.0.last();

    let earliest_take_off_time = match prev_op {
        None => constraints.earliest_time,
        Some(prev_op) => {
            let prev_aircraft_idx = prev_op.aircraft_idx;

            let separation = instance
                .separation(prev_aircraft_idx, aircraft_idx)
                .unwrap();

            constraints
                .earliest_time
                .max(prev_op.schedule.op_time() + separation)
        },
    };

    let last_dep = schedules.0.iter().rev().find_map(|op| match &op.schedule {
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
    schedules: &RunwaySchedule,
    instance: &Instance,
) -> Op {
    let prev_aircraft_idx = schedules.0.last().map(|op| op.aircraft_idx);

    let landing_time = match prev_aircraft_idx {
        None => constraints.earliest_time,
        Some(prev_aircraft_idx) => {
            let prev_earliest_time = instance.rows()[prev_aircraft_idx]
                .constraints
                .earliest_time();

            let separation = instance
                .separation(prev_aircraft_idx, aircraft_idx)
                .unwrap();

            constraints
                .earliest_time
                .max(prev_earliest_time + separation)
        },
    };

    let schedule = OpSchedule::Arrival(ArrivalSchedule { landing_time });

    Op {
        aircraft_idx,
        schedule,
    }
}
