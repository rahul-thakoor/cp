use std::fs::File;
use std::io::Read;
use toml;
use cargo_name::{Availability};
use license_exprs::{validate_license_expr};

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

    pub fn get_license(self) -> Option<String>{
        self.license
    }

    /// Check if the name provided on the Cargo.toml file is available in crates.io
    // TODO: better error handling
    pub fn validate_name(&self){
        match cargo_name::get(&self.name).unwrap() {
            Availability::Available => println!("cargo name valid"),
            Availability::Unavailable => println!("Unavailable."),
            Availability::Unknown => println!("Unknown status code returned."),

        };

    }
    /// Check the license field in Cargo.toml
    pub fn validate_license(&self){
        match &self.license {
            Some(l) => verify_license(&l),
            None => println!("No license"),

        }


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

/// Validate the given license expression
fn verify_license(l:&str){
    match validate_license_expr(l){
        Ok(_) => println!("License {} ok", l),
        Err(e) => println!("{}",e)
    }
}