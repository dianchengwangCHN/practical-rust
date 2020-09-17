use clap::{App, Arg};
use kvs::{KvStore, Result, KvsError};
use std::env::current_dir;

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
        ("get", Some(matches)) => unimplemented!("unimplemented"),
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let value = matches.value_of("VALUE").expect("VALUE argument missing");

            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())
        }
        ("rm", Some(matches)) => unimplemented!("unimplemented"),
        _ => panic!(),
    }
}
