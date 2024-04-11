use std::{fs, num::NonZeroUsize, path::Path};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use runseq::{
    branch_bound::{BranchBound, DeiceStrategy},
    instance::{solve::Solve, Instance},
};

fn furini(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("furini");

    const HORIZON: Option<NonZeroUsize> = NonZeroUsize::new(10);

    // NOTE: Decomposed de-icing fails on instance FPT01
    const DECOMPOSED_FURINI_INSTANCES: &[usize] = &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let branch_bound = BranchBound {
        horizon: HORIZON,
        deice_strategy: DeiceStrategy::ByTobt,
    };

    for &id in DECOMPOSED_FURINI_INSTANCES {
        let instance = load_instance(format!("../instances/furini/toml/{}.toml", id));
        group.bench_with_input(
            BenchmarkId::new("decomposed de-icing", id),
            &instance,
            |bencher, instance| bencher.iter(|| branch_bound.solve(instance)),
        );
    }

    const INTEGRATED_FURINI_INSTANCES: &[usize] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let branch_bound = BranchBound {
        horizon: HORIZON,
        deice_strategy: DeiceStrategy::Integrated,
    };

    for &id in INTEGRATED_FURINI_INSTANCES {
        let instance = load_instance(format!("../instances/furini/toml/{}.toml", id));
        group.bench_with_input(
            BenchmarkId::new("integrated de-icing", id),
            &instance,
            |bencher, instance| bencher.iter(|| branch_bound.solve(instance)),
        );
    }
}

fn heathrow(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("heathrow");

    const HORIZON: Option<NonZeroUsize> = NonZeroUsize::new(10);

    // NOTE: De-icing by TOBT fails on instances 21-25
    const TOBT_HEATHROW_INSTANCES: &[usize] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 26, 27, 28, 29, 30,
    ];

    let branch_bound = BranchBound {
        horizon: HORIZON,
        deice_strategy: DeiceStrategy::ByTobt,
    };

    for &id in TOBT_HEATHROW_INSTANCES {
        let instance = load_instance(format!("../instances/heathrow/toml/{}.toml", id));
        group.bench_with_input(
            BenchmarkId::new("decomposed de-icing by tobt", id),
            &instance,
            |bencher, instance| bencher.iter(|| branch_bound.solve(instance)),
        );
    }

    // NOTE: De-icing by CTOT fails on instances 21-25
    const CTOT_HEATHROW_INSTANCES: &[usize] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 26, 27, 28, 29, 30,
    ];

    let branch_bound = BranchBound {
        horizon: HORIZON,
        deice_strategy: DeiceStrategy::ByCtot,
    };

    for &id in CTOT_HEATHROW_INSTANCES {
        let instance = load_instance(format!("../instances/heathrow/toml/{}.toml", id));
        group.bench_with_input(
            BenchmarkId::new("decomposed de-icing by ctot", id),
            &instance,
            |bencher, instance| bencher.iter(|| branch_bound.solve(instance)),
        );
    }

    const INTEGRATED_HEATHROW_INSTANCES: &[usize] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30,
    ];

    let branch_bound = BranchBound {
        horizon: HORIZON,
        deice_strategy: DeiceStrategy::Integrated,
    };

    for &id in INTEGRATED_HEATHROW_INSTANCES {
        let instance = load_instance(format!("../instances/heathrow/toml/{}.toml", id));
        group.bench_with_input(
            BenchmarkId::new("integrated de-icing", id),
            &instance,
            |bencher, instance| bencher.iter(|| branch_bound.solve(instance)),
        );
    }
}

fn load_instance(path: impl AsRef<Path>) -> Instance {
    let toml = fs::read_to_string(path).unwrap();
    let instance = toml::from_str::<Instance>(&toml).unwrap();
    instance
}

criterion_group!(benches, furini, heathrow);
criterion_main!(benches);
