use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use rust_xlsxwriter::Workbook;

use irdis::{
    data::{from_heathrow_with_limit, to_xlsx},
    instance::Instance,
};

fn main() {
    let flights = fs::read_to_string("instances/heathrow/original/flights.csv").unwrap();
    let runway_configs =
        fs::read_to_string("instances/heathrow/original/runway-configurations.csv").unwrap();
    let pushback_durs =
        fs::read_to_string("instances/heathrow/original/pushback-durations.csv").unwrap();
    let taxi_configs =
        fs::read_to_string("instances/heathrow/original/taxi-durations.csv").unwrap();
    let separation_configs =
        fs::read_to_string("instances/heathrow/original/runway-separations.csv").unwrap();

    let instances = from_heathrow_with_limit(
        &flights,
        &pushback_durs,
        &taxi_configs,
        &runway_configs,
        &separation_configs,
        4013,
    )
    .unwrap();

    let flight_counts = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 20, 25, 30, 40, 50,
    ];
    for (id, flight_count) in flight_counts.into_iter().enumerate() {
        let instance = instances
            .iter()
            .filter(|instance| instance.flights().len() == flight_count)
            .next()
            .unwrap();

        let toml_path = format!("instances/heathrow/toml/{}.toml", id + 1);
        save_toml(instance, toml_path.as_ref());

        let xlsx_path = format!("instances/heathrow/xlsx/{}.xlsx", id + 1);
        save_xlsx(instance, xlsx_path.as_ref());
    }
}

fn save_toml(instance: &Instance, path: &Path) {
    let toml = toml::to_string(&instance).unwrap();
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    file.write_all(toml.as_bytes()).unwrap();
}

fn save_xlsx(instance: &Instance, path: &Path) {
    let mut workbook = Workbook::new();
    let sheet = to_xlsx(instance).unwrap();
    workbook.push_worksheet(sheet);
    workbook.save(path).unwrap();
}
