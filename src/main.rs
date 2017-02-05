extern crate clap;
extern crate toml;
extern crate rustc_serialize;
mod config;
use std::io::prelude::*;
use std::fs::File;
use std::process;
use std::process::Command;
use toml::{Parser, Value};
use clap::{Arg, App, SubCommand};


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

fn save_configuration(config: config::Config) {
    let toml_string = toml::encode_str(&config);

    let mut file = std::fs::File::create("servers.toml").unwrap();
    file.write_all(toml_string.as_bytes()).expect("Could not write to file!");
}

fn add_to_file(role: String, address: String) {
    let mut config = read_server_file();
    if config.is_role_unique(&role) {
        config.add_role(role, address);
        save_configuration(config);
    } else {
        println!("Role {} is already configured", role);
        process::exit(1)
    }
}

fn deploy(role_name: String) {
    let config = read_server_file();

    if let Err(errors) = config.is_valid() {
        println!("Configuration is not valid");
        for error in errors {
            println!("{}", error);
        }
        process::exit(1)
    }

    match config.address_for_role_name(&role_name) {
        Some(address) => {
            println!("Deploying {} - {}", role_name, address);
            let output = Command::new("knife")
                     .arg("solo")
                     .arg("bootstrap")
                     .arg(format!("{}@{}", config.username, address))
                     .arg("-i")
                     .arg(config.ssh_key_path)
                     .arg("--no-host-key-verify")
                     .arg("--node-name")
                     .arg(role_name)
                     .output()
                     .expect("failed to execute process");
            let stdout = String::from_utf8(output.stdout).unwrap();
            let stderr = String::from_utf8(output.stderr).unwrap();
            if output.status.success() {
                println!("{}", stdout);
            } else {
                println!("{}", stderr);
                process::exit(1)
            }
        },
        None => {
            println!("No address found for role {}", role_name);
            process::exit(1)
        }
    }
}

fn add_username(username: String) {
    let mut config = read_server_file();
    config.add_username(username);
    save_configuration(config);
}

fn add_ssh_key(path: String) {
    let mut config = read_server_file();
    config.add_ssh_key(path);
    save_configuration(config);
}

fn main() {
    let matches = App::new(APP_NAME)
            .version(VERSION)
            .author("Jan Schulte <jan@unexpected-co.de>")
            .about("Deploys your tool -- The captain said I have to")
            .arg(Arg::with_name("list")
                               .short("l")
                               .long("list")
                               .help("Lists all values from the server file")
                               .takes_value(false))
            .subcommand(SubCommand::with_name("add")
              .about("Add a new role")
              .arg(Arg::with_name("address")
                  .index(1)
                  .required(true)
                  .requires("role")
                  .help("Define an address"))
              .arg(Arg::with_name("role")
                  .index(2)
                  .help("Define a role")))
            .subcommand(SubCommand::with_name("deploy")
              .about("Start deployment for a certain role")
              .arg(Arg::with_name("role")
                    .index(1)
                    .required(true)
                    .help("The machine which should be deployed")))
            .subcommand(SubCommand::with_name("add_user")
              .about("Configure username")
              .arg(Arg::with_name("username")
                    .index(1)
                    .required(true)
                    .help("Configures the machine's username")))
            .subcommand(SubCommand::with_name("add_key")
              .about("Configure ssh key")
              .arg(Arg::with_name("key")
                    .index(1)
                    .required(true)
                    .help("Configures path to ssh key")))
            .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("add") {
      let role = matches.value_of("role").unwrap();
      let address = matches.value_of("address").unwrap();
      add_to_file(role.to_string(), address.to_string());
    }
    else if matches.occurrences_of("list") > 0 {
        let config = read_server_file();
        println!("Reading servers.toml...");
        match config.is_valid() {
            Ok(()) => {
                println!("Username: {}", config.username);
                println!("SSH Key path: {}", config.ssh_key_path);
                for rule in config.roles {
                    println!("🖥 {} － {}", rule.name, rule.address);
                }
            },
            Err(errors) => {
                println!("Configuration is not valid!");
                for error in errors {
                    println!("{}", error);
                }
                process::exit(1)
            }
        }
    }
    else if let Some(ref matches) = matches.subcommand_matches("add_user") {
        let username = matches.value_of("username").unwrap();
        println!("Configuring {}", username);
        add_username(username.to_string());
    }
    else if let Some(ref matches) = matches.subcommand_matches("add_key") {
        let key = matches.value_of("key").unwrap();
        println!("Configuring SSH Key {}", key);
        add_ssh_key(key.to_string());
    }
    else if let Some(ref matches) = matches.subcommand_matches("deploy") {
        let role = matches.value_of("role").unwrap();
        deploy(role.to_string());
    }
}
