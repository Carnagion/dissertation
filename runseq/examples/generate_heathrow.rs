use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use rust_xlsxwriter::Workbook;

use runseq::{
    data::{heathrow::from_heathrow, xlsx::to_xlsx},
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

    let instances = from_heathrow(&flights, &pushback_durs, &separation_configs).unwrap();

    let flight_counts = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20, 25, 30, 40, 50,
    ];
    for (id, flight_count) in (1..).zip(flight_counts) {
        let instance = instances
            .iter()
            .filter(|instance| instance.flights().len() == flight_count)
            .max_by_key(|instance| deice_count(instance.flights()))
            .unwrap();

        save_toml(instance, format!("../instances/heathrow/toml/{}.toml", id));
        save_xlsx(instance, format!("../instances/heathrow/xlsx/{}.xlsx", id));
    }

    let large_instances = instances
        .iter()
        .filter(|instance| instance.flights().len() == 55)
        .take(10);
    for (id, instance) in (21..).zip(large_instances) {
        save_toml(instance, format!("../instances/heathrow/toml/{}.toml", id));
        save_xlsx(instance, format!("../instances/heathrow/xlsx/{}.xlsx", id));
    }
}

fn deice_count(flights: &[Flight]) -> usize {
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
    let mut workbook = Workbook::new();
    let sheet = to_xlsx(instance).unwrap();
    workbook.push_worksheet(sheet);
    workbook.save(path).unwrap();
}
