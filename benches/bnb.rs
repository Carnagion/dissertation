use std::{fs, str::FromStr};

use divan::Bencher;

use dissertation::{bnb, instance::Instance};

fn main() {
    divan::main();
}

#[divan::bench(consts = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12])]
fn branch_and_bound<const INSTANCE: usize>(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let path = format!("benches/instances/{}.csv", INSTANCE);
            let instance = fs::read_to_string(path).unwrap();
            Instance::from_str(&instance).unwrap()
        })
        .bench_refs(|instance| bnb::branch_and_bound(instance));
}
