use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use runseq::{
    data::{furini::from_furini_with_limit, xlsx::to_xlsx},
    instance::Instance,
};

fn main() {
    for id in 1..=12 {
        let flights_path =
            Path::new("../instances/furini/original/").join(format!("FPT{:0>2}.txt", id));
        let flights = fs::read_to_string(&flights_path).unwrap();

        let separations_path = Path::new("../instances/furini/original/sep/")
            .join(format!("info_matrix_FPT{:0>2}.txt.txt", id));
        let separations = fs::read_to_string(separations_path).unwrap();

        let instance = from_furini_with_limit(&flights, &separations, 60).unwrap();

        save_toml(&instance, format!("../instances/furini/toml/{}.toml", id));
        save_xlsx(&instance, format!("../instances/furini/xlsx/{}.xlsx", id));
    }
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
