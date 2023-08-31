use clap::{arg, Command};
use config::Config;
use log::{debug, error, info, log_enabled, Level};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

use fs_extra::dir::*;
use fs_extra::error::*;

fn main() {
    env_logger::init();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    let settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("Settings.toml"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        //.add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("config", sub_matches)) => {
            let hashmap = settings
                .try_deserialize::<HashMap<String, String>>()
                .unwrap();

            println!("config as a HashMap \n{:?}", hashmap);
        }
        Some(("use", sub_matches)) => {
            println!(
                "Creating {}",
                sub_matches.get_one::<String>("TEMPLATE").expect("required")
            );

            //get current working directory
            let cwd = std::env::current_dir().unwrap();
            //copy all files from the directory to the current working directory with the name of the template
            let template_dir = PathBuf::from("templates")
                .join(sub_matches.get_one::<String>("TEMPLATE").expect("required"));
            let mut copy_options = fs_extra::dir::CopyOptions::new();
            copy_options.overwrite = true;
            fs_extra::dir::copy(template_dir, cwd, &copy_options).unwrap();

        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}

fn cli() -> Command {
    Command::new("rust-template")
        .about("A template helper tool for Rust projects")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("config")
            .about("Prints out the current configuration")
        )
        .subcommand(
            Command::new("use")
            .about("creates a new project based on a template")
            .arg(arg!(<TEMPLATE> "The name of the template to use"))
            .arg_required_else_help(true),
        )
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
}
