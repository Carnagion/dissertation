use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use rust_xlsxwriter::Workbook;

use irdis::{
    data::{from_furini_with_limit, to_xlsx},
    instance::Instance,
};

fn main() {
    for id in 1..=12 {
        let flights_path =
            Path::new("instances/furini/original").join(format!("FPT{:0>2}.txt", id));
        let flights = fs::read_to_string(&flights_path).unwrap();

        let separations_path = Path::new("instances/furini/original/sep")
            .join(format!("info_matrix_FPT{:0>2}.txt.txt", id));
        let separations = fs::read_to_string(separations_path).unwrap();

        let instance = from_furini_with_limit(&flights, &separations, 10).unwrap();

        let instance_dir = flights_path.parent().unwrap().parent().unwrap();

        let toml_path = instance_dir.join(format!("toml/{}.toml", id));
        save_toml(&instance, &toml_path);

        let xlsx_path = instance_dir.join(format!("xlsx/{}.xlsx", id));
        save_xlsx(&instance, &xlsx_path);
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
