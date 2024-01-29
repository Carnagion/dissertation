use std::fs;

use divan::Bencher;

use irdis_branch_bound::BranchBound;

use irdis_core::instance::Instance;

fn main() {
    divan::main();
}

#[divan::bench(consts = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
fn branch_bound<const INSTANCE: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let path = format!("../instances/modified-furini/{}.csv", INSTANCE);
            let csv = fs::read_to_string(path).unwrap();
            csv.parse::<Instance>().unwrap()
        })
        .bench_refs(|instance| instance.solve::<BranchBound>());
}
