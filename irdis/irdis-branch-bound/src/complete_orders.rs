use irdis_instance::{flight::Flight, Instance};

pub fn separation_identical_sets(instance: &Instance) -> Vec<Vec<usize>> {
    let mut sets = Vec::<Vec<_>>::with_capacity(instance.flights().len().min(1));

    'unclassified: for j in 0..instance.flights().len() {
        'sets: for set in &mut sets {
            let are_complete_ordered = set.iter().copied().all(|i| {
                are_separation_identical(i, j, instance)
                    && (are_complete_ordered(i, j, instance)
                        || are_complete_ordered(j, i, instance))
            });

            if !are_complete_ordered {
                continue 'sets;
            }

            set.push(j);
            continue 'unclassified;
        }

        let set = vec![j];
        sets.push(set);
    }

    for set in &mut sets {
        set.sort_by(|&i, &j| {
            let cmp_idx = i.cmp(&j);
            let i = &instance.flights()[i];
            let j = &instance.flights()[j];
            i.release_time()
                .cmp(&j.release_time())
                .then_with(|| i.base_time().cmp(&j.base_time()))
                .then_with(|| i.time_window().latest.cmp(&j.time_window().latest))
                .then(cmp_idx)
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

fn are_complete_ordered(i: usize, j: usize, instance: &Instance) -> bool {
    let i = &instance.flights()[i];
    let j = &instance.flights()[j];
    match (i, j) {
        (Flight::Arr(i), Flight::Arr(j)) => {
            i.release_time() <= j.release_time()
                && i.base_time <= j.base_time
                && i.window.latest <= j.window.latest
        },
        // NOTE: Complete orders cannot be inferred when one or both aircraft are subject to CTOT windows.
        _ => false,
    }
}
