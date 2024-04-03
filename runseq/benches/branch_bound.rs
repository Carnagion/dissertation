use std::{fs, num::NonZeroUsize, path::Path};

use divan::Bencher;

use runseq::{
    branch_bound::{BranchBound, DeiceStrategy},
    instance::{solve::Solve, Instance},
};

fn main() {
    divan::main();
}

#[divan::bench_group(sample_count = 10, sample_size = 1)]
mod furini {
    use super::*;

    const FURINI_INSTANCES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    const HORIZON: Option<NonZeroUsize> = NonZeroUsize::new(12);

    // NOTE: Decomposed de-icing fails on instances FPT01 and FPT06
    #[divan::bench(args = FURINI_INSTANCES)]
    fn deice_decomposed(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("../instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByTobt,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    #[divan::bench(args = FURINI_INSTANCES)]
    fn deice_integrated(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("../instances/furini/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::Integrated,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }
}

#[divan::bench_group(sample_count = 10, sample_size = 1)]
mod heathrow {
    use super::*;

    const HEATHROW_INSTANCES: &[usize] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30,
    ];

    const HORIZON: Option<NonZeroUsize> = NonZeroUsize::new(10);

    #[divan::bench(args = HEATHROW_INSTANCES)]
    fn deice_by_tobt(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("../instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByTobt,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    #[divan::bench(args = HEATHROW_INSTANCES)]
    fn deice_by_ctot(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("../instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::ByCtot,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }

    #[divan::bench(args = HEATHROW_INSTANCES)]
    fn deice_integrated(bencher: Bencher, instance: usize) {
        let instance = load_instance(format!("../instances/heathrow/toml/{}.toml", instance));

        let branch_bound = BranchBound {
            horizon: HORIZON,
            deice_strategy: DeiceStrategy::Integrated,
        };

        bencher.bench_local(|| {
            branch_bound.solve(&instance);
        });
    }
}

fn load_instance(path: impl AsRef<Path>) -> Instance {
    let toml = fs::read_to_string(path).unwrap();
    let instance = toml::from_str::<Instance>(&toml).unwrap();
    instance
}
