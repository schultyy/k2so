extern crate clap;
extern crate toml;
extern crate rustc_serialize;
mod config;
use clap::{Arg, App, SubCommand};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "K2-SO -- Deployment Droid 🤖✨";

fn add_to_file(role: String, address: String) {
    let config = config::Config::new(role, address);
    let result = toml::encode_str(&config);

    println!("{:?}", result);
}

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
      let role = matches.value_of("role").unwrap();
      let address = matches.value_of("address").unwrap();
      add_to_file(role.to_string(), address.to_string());
    }
}
