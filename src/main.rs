extern crate clap;
use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "K2-SO -- Deployment Droid 🤖✨";

fn main() {
    let matches = App::new(APP_NAME)
            .version(VERSION)
            .author("Jan Schulte <jan@unexpected-co.de>")
            .about("Deploys your tool -- The captain said I have to")
            .arg(Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("IP")
                .help("Define an address"))
            .arg(Arg::with_name("role")
                .short("r")
                .long("role")
                .value_name("ROLE")
                .help("Define a role"))
            .get_matches();
    let ip = matches.value_of("address").unwrap();
    let role = matches.value_of("role").unwrap();
    println!("Value for config: {}", ip);
    println!("Value for config: {}", role);
}
