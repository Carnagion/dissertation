use std::{num::NonZeroUsize, ops::Range};

use irdis_core::{
    instance::{schedule::RunwaySchedule, Instance, SeparationSets},
    solve::Solve,
};

mod bound;
use bound::{estimated_remaining_bound, op_cost, Bounds};

mod schedule;
use schedule::schedule_aircraft;

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
