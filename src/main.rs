extern crate cargopublish;

use cargopublish::*;



fn main() {

    println!("Loading crate info ...");
    let package = get_package_info("/Users/rahul/Documents/Github/cp");

    validate_name(package.get_package_name());

}