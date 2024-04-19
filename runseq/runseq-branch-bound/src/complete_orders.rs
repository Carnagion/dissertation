use std::cmp::Ordering;

use runseq_instance::{flight::Flight, Instance};

pub fn separation_identical_complete_orders(instance: &Instance) -> Vec<Vec<usize>> {
    let mut sets = Vec::<Vec<_>>::with_capacity(instance.flights().len().min(1));

    // Goes through all pairs of aircraft and checks if they are separation-identical.
    // If they are, they are put in the first set for which a complete order exists between all of them.
    'unclassified: for j in 0..instance.flights().len() {
        'sets: for set in &mut sets {
            let separation_identical_complete_order = set.iter().copied().all(|i| {
                are_separation_identical(i, j, instance)
                    && (complete_order_exists(i, j, instance)
                        || complete_order_exists(j, i, instance))
            });

            if !separation_identical_complete_order {
                continue 'sets;
            }

            set.push(j);
            continue 'unclassified;
        }

        let set = vec![j];
        sets.push(set);
    }

    // The aircraft in each set are sorted in ascending order according to their release times,
    // base times, and latest times in their time windows.
    // If two aircraft have the same values for all of these, the one with the lower index (which will
    // always be unique) is put first.
    for set in &mut sets {
        set.sort_unstable_by(|&flight_idx, &other_idx| {
            let flight = &instance.flights()[flight_idx];
            let other = &instance.flights()[other_idx];
            flight
                .release_time()
                .cmp(&other.release_time())
                .then_with(|| flight.base_time().cmp(&other.base_time()))
                .then_with(|| cmp_latest(flight, other))
                .then_with(|| flight_idx.cmp(&other_idx))
        })
    }

    sets
}

fn are_separation_identical(i: usize, j: usize, instance: &Instance) -> bool {
    (0..instance.flights().len())
        .filter(|&k| i != k && j != k)
        .all(|k| {
            let sep_i_k = instance.separations().get(i, k);
            let sep_k_i = instance.separations().get(k, i);

            let sep_j_k = instance.separations().get(j, k);
            let sep_k_j = instance.separations().get(k, j);

            sep_i_k == sep_j_k && sep_k_i == sep_k_j
        })
}

fn complete_order_exists(flight_idx: usize, other_idx: usize, instance: &Instance) -> bool {
    let flight = &instance.flights()[flight_idx];
    let other = &instance.flights()[other_idx];

    // NOTE: Complete orders cannot be inferred when one or both flights are subject to CTOTs.
    has_no_ctot(flight)
        && has_no_ctot(other)
        && flight.release_time() <= other.release_time()
        && flight.base_time() <= other.base_time()
        && cmp_latest(flight, other).is_le()
}

fn has_no_ctot(flight: &Flight) -> bool {
    flight
        .as_departure()
        .and_then(|dep| dep.ctot.as_ref())
        .is_none()
}

fn cmp_latest(flight: &Flight, other: &Flight) -> Ordering {
    match (flight.window(), other.window()) {
        // The latest time of an aircraft does not matter if the aircraft being compared has no time window.
        (_, None) => Ordering::Equal,
        // The latest time of an aircraft that has no time window is always greater than the latest time of any other aircraft.
        (None, Some(_)) => Ordering::Greater,
        (Some(flight_window), Some(other_window)) => {
            flight_window.latest().cmp(&other_window.latest())
        },
    }
}
