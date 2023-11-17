use crate::instance::{AssignedOp, Instance, OpConstraint};

pub fn branch_and_bound(instance: &Instance) {
    let sets = instance.separation_sets();
    let mut seq = Vec::with_capacity(instance.aircraft.len());
    let mut set_idxs = vec![0; sets.len()];
    let mut min_cost = f32::INFINITY;
    branch_with_depth(0, &mut seq, &mut set_idxs, &sets, instance, &mut min_cost);
    println!("{}", min_cost);
}

fn branch_with_depth(
    depth: usize,
    seq: &mut Vec<AssignedOp>,
    set_idxs: &mut [usize],
    sets: &[Vec<usize>],
    instance: &Instance,
    min_cost: &mut f32,
) {
    if depth >= instance.aircraft.len() {
        let current_cost = cost(&seq, depth);
        if current_cost < *min_cost {
            *min_cost = current_cost;
        }

        debug_seq(&seq, depth);

        return;
    }

    for set_idx in 0..sets.len() {
        if set_idxs[set_idx] >= sets[set_idx].len() {
            continue;
        }

        let aircraft_idx = sets[set_idx][set_idxs[set_idx]];
        let OpConstraint {
            kind,
            earliest_time,
            ..
        } = instance.op_constraints[aircraft_idx];

        let assigned_time = match depth {
            0 => earliest_time,
            depth => {
                let prev_op = seq[depth - 1];
                let separation = instance
                    .separation(aircraft_idx, prev_op.aircraft_idx)
                    .unwrap(); // PANICS: The indices will definitely be valid
                (prev_op.time + separation).max(earliest_time)
            }
        };

        let assigned_op = AssignedOp {
            aircraft_idx,
            kind,
            earliest_time,
            time: assigned_time,
        };

        if depth >= seq.len() {
            seq.push(assigned_op);
        } else {
            seq[depth] = assigned_op;
        }

        debug_seq(&seq, depth);

        let current_cost = cost(&seq, depth);
        if current_cost > *min_cost {
            continue;
        }

        set_idxs[set_idx] += 1;
        branch_with_depth(depth + 1, seq, set_idxs, sets, instance, min_cost);
        set_idxs[set_idx] -= 1;
    }
}

fn cost(seq: &[AssignedOp], depth: usize) -> f32 {
    seq.into_iter()
        .take(depth + 1)
        .map(|op| (op.time - op.earliest_time).as_seconds_f32().powi(2))
        .sum()
}

fn debug_seq(seq: &[AssignedOp], depth: usize) {
    print!("[");
    for op in seq.iter().take(depth) {
        print!(
            "{} ({} vs {}), ",
            op.aircraft_idx, op.time, op.earliest_time,
        );
    }
    println!("] at {} scoring {}", depth, cost(seq, depth));
}
