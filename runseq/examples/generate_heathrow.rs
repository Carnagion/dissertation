use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use runseq::{
    data::{
        heathrow::{from_heathrow, from_heathrow_with_limits},
        xlsx::to_xlsx,
    },
    instance::{flight::Flight, Instance},
};

fn main() {
    // NOTE: `include_str!` is not used here since the data is confidential and compile errors
    //        when trying to run examples are annoying.
    let flights = fs::read_to_string("../instances/heathrow/original/flights.csv").unwrap();
    let pushback_durs =
        fs::read_to_string("../instances/heathrow/original/pushback-durations.csv").unwrap();
    let separation_configs =
        fs::read_to_string("../instances/heathrow/original/runway-separations.csv").unwrap();

    let small_instances = from_heathrow_with_limits(
        &flights,
        &pushback_durs,
        &separation_configs,
        usize::MAX,
        10,
    )
    .unwrap();

    for (id, deice_count) in (1..=5).zip([0, 1, 2, 3, 4]) {
        let instance = small_instances
            .iter()
            .find(|instance| {
                flights_to_deice(instance.flights()) == deice_count
                    && instance.flights().len() == 10
            })
            .unwrap();

        save_toml(instance, format!("../instances/heathrow/toml/{}.toml", id));
        save_xlsx(instance, format!("../instances/heathrow/xlsx/{}.xlsx", id));
    }

    let upper_small_instances = small_instances
        .iter()
        .filter(|instance| {
            flights_to_deice(instance.flights()) == 5 && instance.flights().len() == 10
        })
        .take(5);
    for (id, instance) in (6..=10).zip(upper_small_instances) {
        save_toml(instance, format!("../instances/heathrow/toml/{}.toml", id));
        save_xlsx(instance, format!("../instances/heathrow/xlsx/{}.xlsx", id));
    }

    let medium_large_instances =
        from_heathrow(&flights, &pushback_durs, &separation_configs).unwrap();

    for (id, size) in (11..).step_by(2).zip([15, 25, 35, 45, 55]) {
        let mut instances = medium_large_instances
            .iter()
            .filter(|instance| instance.flights().len() == size);

        let instance = instances.next().unwrap();
        save_toml(instance, format!("../instances/heathrow/toml/{}.toml", id));
        save_xlsx(instance, format!("../instances/heathrow/xlsx/{}.xlsx", id));

        let instance = instances.next().unwrap();
        save_toml(
            instance,
            format!("../instances/heathrow/toml/{}.toml", id + 1),
        );
        save_xlsx(
            instance,
            format!("../instances/heathrow/xlsx/{}.xlsx", id + 1),
        );
    }

    let large_instances = medium_large_instances
        .iter()
        .filter(|instance| instance.flights().len() == 60);
    for (id, instance) in (21..=30).zip(large_instances) {
        save_toml(instance, format!("../instances/heathrow/toml/{}.toml", id));
        save_xlsx(instance, format!("../instances/heathrow/xlsx/{}.xlsx", id));
    }
}

fn flights_to_deice(flights: &[Flight]) -> usize {
    flights
        .iter()
        .filter_map(|flight| flight.as_departure()?.deice.as_ref())
        .count()
}

fn save_toml(instance: &Instance, path: impl AsRef<Path>) {
    let toml = toml::to_string(&instance).unwrap();
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    file.write_all(toml.as_bytes()).unwrap();
}

fn save_xlsx(instance: &Instance, path: impl AsRef<Path>) {
    let mut workbook = to_xlsx(instance).unwrap();
    workbook.save(path).unwrap();
}
