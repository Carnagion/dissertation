use time::Time;

use crate::instance::{AssignedOp, Instance, OpConstraint};

pub fn branch_and_bound(instance: &Instance) {
    let sets = instance.separation_sets();
    let mut seq = vec![None; instance.aircraft.len()];
    let mut set_idxs = vec![0; sets.len()];
    branch_with_depth(0, &mut seq, &mut set_idxs, &sets, instance)
}

fn branch_with_depth(
    depth: usize,
    seq: &mut [Option<AssignedOp>],
    set_idxs: &mut [usize],
    sets: &[Vec<usize>],
    instance: &Instance,
) {
    if depth == seq.len() {
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
        } = instance.op_constraints[aircraft_idx];

        // TODO: Assign a time to the aircraft
        seq[depth] = Some(AssignedOp {
            aircraft_idx,
            kind,
            earliest_time,
            time: earliest_time,
        });

        set_idxs[set_idx] += 1;
        branch_with_depth(depth + 1, seq, set_idxs, sets, instance);
        set_idxs[set_idx] -= 1;
    }
}
