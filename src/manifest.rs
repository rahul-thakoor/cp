use std::fs::File;
use std::io::Read;
use toml;
use cargo_name::{Availability};

#[derive(Debug,Deserialize)]
struct CargoManifest {
    package: CargoPackage,
}


#[derive(Debug,Deserialize)]
pub struct CargoPackage {
    name : String,
    authors: Vec<String>,
    description: Option<String>,
    version: String,
    license: Option<String>,
    repository: Option<String>,
}

impl CargoPackage{
    pub fn get_package_name(self) -> String{
        self.name
    }

}




/// Read Cargo.toml file from provided path
/// Path should be crate root
fn read_cargo_toml(path: &str) -> CargoManifest{
    let manifest_path = format!("{}/Cargo.toml", path);
    let mut cargo_file = File::open(manifest_path).expect("Cannot open manifest file");
    let mut cargo_contents = String::new();
    cargo_file.read_to_string(&mut cargo_contents).unwrap();
    let decoded: CargoManifest = toml::from_str(&cargo_contents).unwrap();
    return decoded;
}

pub fn get_package_info(path: &str) -> CargoPackage {
    read_cargo_toml(path).package

}

/// Check if the name provided on the Cargo.toml file is available in crates.io
pub fn validate_name(name:String){
    match cargo_name::get(&name).unwrap() {
        Availability::Available => println!("cargo name valid"),
        Availability::Unavailable => println!("Unavailable."),
        Availability::Unknown => println!("Unknown status code returned."),

    };

}