use std::{fs, str::FromStr};

use divan::Bencher;

use dissertation::{bnb, instance::Instance};

fn main() {
    divan::main();
}

#[divan::bench(consts = [1, 2, 3, 4, 5, 6])]
fn branch_and_bound<const INSTANCE: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let instance =
                fs::read_to_string(format!("instances/instance-{}.csv", INSTANCE)).unwrap();
            Instance::from_str(&instance).unwrap()
        })
        .bench_refs(|instance| bnb::branch_and_bound(instance));
}
