use std::fs::File;
use std::io::Read;
use toml;
use reqwest;
use serde_json::*;

static  CARGO_API_ENDPOINT: &str = "https://crates.io/api/v1/crates/";

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

pub fn validate_name(name:String){
    let endpoint = format!("{}{}", CARGO_API_ENDPOINT, name);
    let mut res = reqwest::get(&endpoint).unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&body).expect("Cannot parse reqwest data");



    println!("Body:\n{}", body);
    println!("Parsed data: {}", v["errors"] );
}