use std::{fs, num::NonZeroUsize};

use divan::Bencher;

use irdis::{instance::Instance, solve::BranchBound, vis::Visualiser};

fn main() {
    divan::main();
}

#[divan::bench(consts = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
fn visualise_branch_bound<const INSTANCE: usize>(bencher: Bencher) {
    let branch_bound = BranchBound::with_rolling_horizon(NonZeroUsize::new(10).unwrap());
    let vis = Visualiser::new();
    bencher
        .with_inputs(|| {
            let path = format!("instances/furini/limited/{}.csv", INSTANCE);
            let csv = fs::read_to_string(path).unwrap();
            let instance = csv.parse::<Instance>().unwrap();
            (instance.solve_with(&branch_bound), instance)
        })
        .bench_refs(|(schedule, instance)| vis.visualise(&schedule.0, &instance))
}
