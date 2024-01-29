use std::time::Duration;

use irdis_core::instance::{
    op::OpConstraints,
    schedule::{ArrivalSchedule, DepartureSchedule, Op, OpSchedule},
    Instance,
    SeparationSets,
};

#[derive(Debug)]
pub(super) struct Bounds {
    pub(super) lowest: f64,
    pub(super) current: f64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            lowest: f64::INFINITY,
            current: 0.0,
        }
    }
}

pub(super) fn op_cost(op: &Op, instance: &Instance) -> f64 {
    let earliest_time = instance.rows()[op.aircraft_idx].constraints.earliest_time();
    let actual_time = op.schedule.op_time();
    let diff = (actual_time - earliest_time).num_minutes() as f64;
    diff.powi(2)
}

pub(super) fn estimated_remaining_bound(
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

fn bound<S>(schedule: S, instance: &Instance) -> f64
where
    S: IntoIterator<Item = Op>,
{
    schedule.into_iter().map(|op| op_cost(&op, instance)).sum()
}
