extern crate clap;
use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "K2-SO -- Deployment Droid 🤖✨";

fn main() {
    let matches = App::new(APP_NAME)
            .version(VERSION)
            .author("Jan Schulte <jan@unexpected-co.de>")
            .about("Deploys your tool -- The captain said I have to")
            .subcommand(SubCommand::with_name("add")
              .arg(Arg::with_name("address")
                  .index(1)
                  .required(true)
                  .requires("role")
                  .help("Define an address"))
              .arg(Arg::with_name("role")
                  .index(2)
                  .help("Define a role"))
            ).get_matches();
    if let Some(ref matches) = matches.subcommand_matches("add") {
      println!("Role: {}", matches.value_of("role").unwrap());
      println!("Address: {}", matches.value_of("address").unwrap());
    }
}
