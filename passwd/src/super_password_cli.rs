extern crate oath;
extern crate clap;

use clap::{Arg, App};

mod generate_password;

fn main() {
    let matchs = App::new("tx super password generator")
        .arg(Arg::with_name("orgid")
            .long("orgid")
            .value_name("ID")
            .help("input organization ID")
            .takes_value(true)
            .required(true))
        .get_matches();

    let org_id = matchs.value_of("orgid").unwrap();

    let super_password = generate_password::generate_super_password(&org_id);
    
    println!("password: {}", super_password);
}
