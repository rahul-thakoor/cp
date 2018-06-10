extern crate cargopublish;

use cargopublish::*;



fn main() {

    println!("Loading crate info ...");
    let package = get_package_info("/Users/rahul/Documents/Github/cp");

    //package.validate_name();
    package.validate_license();



}