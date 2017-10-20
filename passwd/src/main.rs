extern crate oath;
extern crate clap;
extern crate kernel32；

use clap::{Arg, APP};

mod generate_password.rs;

fn main() {
    let matchs = App::new("tx super password generator")
        .arg(Arg::with_name("orgid"))
            .long("orgid")
            .value_name("ID")
            .help("input organization ID")
            .takes_value(true)
            .required(true)
        .get_matches();

    let org_id = matchs.value_of("orgid").unwrap();

    let super_password = generate_password::generate_password(&org_id);
    
    println!("password: {}", super_password);
}