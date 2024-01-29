use irdis_core::instance::{
    op::{ArrivalConstraints, DepartureConstraints, OpConstraints},
    schedule::{ArrivalSchedule, DepartureSchedule, Op, OpSchedule, RunwaySchedule},
    Instance,
};

pub(super) fn schedule_aircraft(
    aircraft_idx: usize,
    schedule: &RunwaySchedule,
    instance: &Instance,
) -> Op {
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
