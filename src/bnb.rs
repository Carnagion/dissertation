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
    let mut bounds = Bounds::default();
    let mut best_sequence = sequence.clone();
    branch(
        instance,
        &separation_sets,
        &mut sequence,
        &mut last_set_indices,
        &mut bounds,
        &mut best_sequence,
    );
    best_sequence
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    pub lowest: f64,
    pub current_lower: f64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            lowest: f64::INFINITY,
            current_lower: 0.0,
        }
    }
}

fn branch(
    instance: &Instance,
    separation_sets: &[Vec<usize>],
    sequence: &mut Vec<Departure>,
    last_set_indices: &mut [usize],
    bounds: &mut Bounds,
    best_sequence: &mut Vec<Departure>,
) {
    if sequence.len() == instance.rows().len() {
        // Update the cost with that of the best sequence found so far
        bounds.lowest = bounds.lowest.min(bounds.current_lower);
        *best_sequence = sequence.clone();

        return;
    }

    for (set_idx, separation_set) in separation_sets.iter().enumerate() {
        // Continue to the next set if the tracking index for the current set has reached the set's length
        let last_set_idx = last_set_indices[set_idx];
        if last_set_idx >= separation_set.len() {
            continue;
        }

        // Calculate the departure for the current aircraft
        let aircraft_idx = separation_set[last_set_idx];
        let departure = schedule_departure(instance, aircraft_idx, sequence);

        // Calculate the cost for the current scheduled departure and its effect on the bound
        let current_cost = departure_cost(&departure, instance);
        let current_bound = bounds.current_lower + current_cost;

        // Avoid exploring sub-branches if the lower bound of this branch is higher than the lowest bound
        // i.e. it cannot produce a better solution than the known worst solution
        if current_bound > bounds.lowest {
            continue;
        }

        // Update the sequence, bounds, and indices
        sequence.push(departure);
        bounds.current_lower = current_bound;
        last_set_indices[set_idx] += 1;

        // Branch on further sequences
        branch(
            instance,
            separation_sets,
            sequence,
            last_set_indices,
            bounds,
            best_sequence,
        );

        // Reset the sequence, bounds, and indices to what they were before
        sequence.pop();
        bounds.current_lower -= current_cost;
        last_set_indices[set_idx] -= 1;
    }
}

fn bound<'dep, S>(instance: &Instance, sequence: S) -> f64
where
    S: IntoIterator<Item = &'dep Departure>,
{
    sequence
        .into_iter()
        .map(|departure| departure_cost(&departure, instance))
        .sum()
}

fn departure_cost(departure: &Departure, instance: &Instance) -> f64 {
    let earliest_take_off_time = instance.rows()[departure.aircraft_idx]
        .constraints
        .earliest_time;
    let diff = (departure.take_off_time - earliest_take_off_time).num_minutes() as f64;
    diff.powi(2)
}

// TODO: Estimate the bound for remaining unsequenced aircraft
fn estimated_remaining_bound(
    instance: &Instance,
    separation_sets: &[Vec<usize>],
    last_set_indices: &[usize],
) -> f64 {
    todo!()
}

fn schedule_departure(
    instance: &Instance,
    aircraft_idx: usize,
    sequence: &[Departure],
) -> Departure {
    // Get the constraints for the current aircraft
    let constraints = &instance.rows()[aircraft_idx].constraints;

    // Assign a de-ice time and departure time for the current aircraft being considered
    let (de_ice_time, take_off_time) = match sequence.last() {
        // If there is no previous departure, this is the first aircraft
        // i.e. its de-icing and take-off time are the earliest possible
        None => (constraints.target_de_ice_time(), constraints.earliest_time),
        Some(prev_dep) => {
            // Calculate the required separation between the current and previous aircraft
            let separation = instance
                .separation(prev_dep.aircraft_idx, aircraft_idx)
                .unwrap(); // PANICS: The indices will definitely be valid

            // Get the constraints for the previous departure
            let prev_constraints = &instance.rows()[prev_dep.aircraft_idx].constraints;

            // The de-ice time is the maximum of when that the aircraft can get there
            // and when the previous aircraft finishes de-icing
            let de_ice_time = constraints
                .target_de_ice_time()
                .max(prev_dep.de_ice_time + prev_constraints.de_ice_dur);

            // The take-off time is the max of its earliest time,
            // the time of the previous take-off plus separation,
            // and the de-ice time plus the duration taken to get to the runway
            let take_off_time = (prev_dep.take_off_time + separation)
                .max(constraints.earliest_time)
                .max(
                    de_ice_time
                        + constraints.de_ice_dur
                        + constraints.post_de_ice_dur
                        + constraints.lineup_dur,
                );

            (de_ice_time, take_off_time)
        }
    };

    Departure {
        aircraft_idx,
        de_ice_time,
        take_off_time,
    }
}
