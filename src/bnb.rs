use chrono::NaiveTime;

use crate::instance::Instance;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Departure {
    pub aircraft_idx: usize,
    pub de_ice_time: NaiveTime,
    pub take_off_time: NaiveTime,
}

pub fn branch_and_bound(instance: &Instance) -> Vec<Departure> {
    let separation_sets = instance.separation_sets();
    let mut sequence = Vec::with_capacity(instance.rows().len());
    let mut last_set_indices = vec![0; separation_sets.len()];
    let mut lower_bound = f64::INFINITY;
    branch(
        instance,
        &separation_sets,
        &mut sequence,
        &mut last_set_indices,
        &mut lower_bound,
        0,
    );
    sequence
}

fn branch(
    instance: &Instance,
    separation_sets: &[Vec<usize>],
    sequence: &mut Vec<Departure>,
    last_set_indices: &mut [usize],
    lowest_bound: &mut f64,
    depth: usize,
) {
    if depth >= instance.rows().len() {
        // Update the cost with that of the best sequence found so far
        let current_low_bound = bound(instance, sequence, depth);
        if current_low_bound < *lowest_bound {
            *lowest_bound = current_low_bound;
        }

        return;
    }

    for (set_idx, separation_set) in separation_sets.iter().enumerate() {
        // Continue to the next set if the tracking index for the current set has reached the set's length
        let last_set_idx = last_set_indices[set_idx];
        if last_set_idx >= separation_set.len() {
            continue;
        }

        // Insert or update the departure for the current aircraft
        let aircraft_idx = separation_set[last_set_idx];
        let departure = schedule_departure(instance, aircraft_idx, sequence, depth);
        if depth >= sequence.len() {
            sequence.push(departure);
        } else {
            sequence[depth] = departure;
        }

        // Avoid exploring sub-branches if the lower bound of this branch is higher than the lowest bound
        // i.e. it cannot produce a better solution than the known worst solution
        let current_low_bound = bound(instance, sequence, depth);
        if current_low_bound > *lowest_bound {
            continue;
        }

        // Branch on further sequences
        last_set_indices[set_idx] += 1;
        branch(
            instance,
            separation_sets,
            sequence,
            last_set_indices,
            lowest_bound,
            depth + 1,
        );
        last_set_indices[set_idx] -= 1;
    }
}

fn bound(instance: &Instance, sequence: &[Departure], depth: usize) -> f64 {
    sequence
        .iter()
        .take(depth + 1)
        .map(|op| {
            let minutes = (op.take_off_time
                - instance.rows()[op.aircraft_idx].constraints.earliest_time)
                .num_minutes() as f64;
            minutes.powi(2)
        })
        .sum()
}

fn schedule_departure(
    instance: &Instance,
    aircraft_idx: usize,
    sequence: &[Departure],
    depth: usize,
) -> Departure {
    // Grab the constraints for the current aircraft
    let constraints = &instance.rows()[aircraft_idx].constraints;

    // Assign a departure time for the current aircraft being considered
    let take_off_time = match depth.checked_sub(1).and_then(|depth| sequence.get(depth)) {
        None => constraints.earliest_time,
        Some(prev_constraints) => {
            let separation = instance
                .separation(prev_constraints.aircraft_idx, aircraft_idx)
                .unwrap(); // PANICS: The indices will definitely be valid
            (prev_constraints.take_off_time + separation).max(constraints.earliest_time)
        }
    };

    Departure {
        aircraft_idx,
        de_ice_time: take_off_time, // TODO: Assign de-icing time
        take_off_time,
    }
}
