use std::{fs, num::NonZeroUsize};

use divan::Bencher;

use irdis_branch_bound::BranchBound;

use irdis_core::instance::Instance;

fn main() {
    divan::main();
}

mod branch_bound {
    use super::*;

    #[divan::bench(consts = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
    fn single_pass_limited<const INSTANCE: usize>(bencher: Bencher) {
        bencher
            .with_inputs(|| {
                let path = format!("instances/furini/limited/{}.csv", INSTANCE);
                let csv = fs::read_to_string(path).unwrap();
                csv.parse::<Instance>().unwrap()
            })
            .bench_refs(|instance| instance.solve::<BranchBound>());
    }

    #[divan::bench(consts = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
    fn rolling_horizon<const INSTANCE: usize>(bencher: Bencher) {
        let branch_bound = BranchBound::with_rolling_horizon(NonZeroUsize::new(10).unwrap());
        bencher
            .with_inputs(|| {
                let path = format!("instances/furini/converted/{}.csv", INSTANCE);
                let csv = fs::read_to_string(path).unwrap();
                csv.parse::<Instance>().unwrap()
            })
            .bench_refs(|instance| instance.solve_with(&branch_bound));
    }
}
