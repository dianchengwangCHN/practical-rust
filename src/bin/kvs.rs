use clap::{App, Arg};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            App::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").about("A string key").required(true)),
        )
        .subcommand(
            App::new("set")
                .about("Set the string value of a given string key")
                .arg(Arg::with_name("KEY").about("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .about("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("rm")
                .about("Remove the given key")
                .arg(Arg::with_name("KEY").about("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");

            let store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            Ok(())
        }
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let value = matches.value_of("VALUE").expect("VALUE argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => Ok(()),
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e)
            }
        }
        _ => panic!(),
    }
}
