use crate::instance::{AssignedOp, Instance, OpConstraint};

pub fn branch_and_bound(instance: &Instance) {
    let sets = instance.separation_sets();
    let mut seq = Vec::with_capacity(instance.aircraft.len());
    let mut set_idxs = vec![0; sets.len()];
    branch_with_depth(0, &mut seq, &mut set_idxs, &sets, instance)
}

fn branch_with_depth(
    depth: usize,
    seq: &mut Vec<AssignedOp>,
    set_idxs: &mut [usize],
    sets: &[Vec<usize>],
    instance: &Instance,
) {
    if depth >= instance.aircraft.len() {
        println!("{:?}", seq);
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
                    .separation(prev_op.aircraft_idx, aircraft_idx)
                    .unwrap();
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

        set_idxs[set_idx] += 1;
        branch_with_depth(depth + 1, seq, set_idxs, sets, instance);
        set_idxs[set_idx] -= 1;
    }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=844ef5ce7ea547d14ebdf526e14fa90b

// pub fn branch_and_bound_breadth_first(instance: &Instance) -> Option<Vec<usize>> {
//     let mut sequences = VecDeque::from([vec![]]);

//     let mut best = None;
//     let mut upper_bound = f32::INFINITY;

//     while let Some(sequence) = sequences.pop_front() {
//         let current_bound = bound(&sequence, instance);

//         if current_bound < upper_bound {
//             upper_bound = current_bound;
//             best = Some(sequence.clone());
//         }

//         for sub_sequence in branch(&sequence, instance) {
//             let sub_bound = bound(&sub_sequence, instance);
//             if sub_bound > current_bound {
//                 continue;
//             }
//             sequences.push_back(sub_sequence);
//         }
//     }

//     best
// }

// fn branch(sequence: &[usize], instance: &Instance) -> impl IntoIterator<Item = Vec<usize>> {
//     []
// }

// fn bound(sequence: &[usize], instance: &Instance) -> f32 {
//     todo!()
// }
