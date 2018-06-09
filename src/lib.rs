#![feature(extern_prelude)]

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate reqwest;
extern crate serde_json;
extern crate cargo_name;

pub use self::manifest::*;
pub mod manifest;