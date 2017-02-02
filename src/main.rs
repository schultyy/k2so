extern crate clap;
extern crate toml;
extern crate rustc_serialize;
mod config;
use clap::{Arg, App, SubCommand};
use std::io::prelude::*;
use std::fs::File;
use toml::{Parser, Value};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_NAME: &'static str = "K2-SO -- Deployment Droid 🤖✨";

fn read_server_file() -> config::Config {
    let mut config_toml = String::new();
    let path = "servers.toml";

    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_)  => {
            return config::Config::new();
        }
    };

    file.read_to_string(&mut config_toml)
            .unwrap_or_else(|err| panic!("Error while reading config: [{}]", err));

    let mut parser = Parser::new(&config_toml);
    let toml = parser.parse();

    if toml.is_none() {
        for err in &parser.errors {
            let (loline, locol) = parser.to_linecol(err.lo);
            let (hiline, hicol) = parser.to_linecol(err.hi);
            println!("{}:{}:{}-{}:{} error: {}",
                     path, loline, locol, hiline, hicol, err.desc);
        }
        panic!("Exiting server");
    }

    let config = Value::Table(toml.unwrap());
    match toml::decode(config) {
        Some(t) => t,
        None => panic!("Error while deserializing config")
    }
}

fn add_to_file(role: String, address: String) {
    let mut config = read_server_file();
    config.add_role(role, address);
    let toml_string = toml::encode_str(&config);

    let mut file = std::fs::File::create("servers.toml").unwrap();
    file.write_all(toml_string.as_bytes()).expect("Could not write to file!");
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
