use clap::{App, Arg};

fn main() {
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
            )
        )
        .subcommand(
            App::new("rm")
            .about("Remove the given key")
            .arg(Arg::with_name("KEY").about("A string key").required(true))
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => unimplemented!("unimplemented"),
        Some("set") => unimplemented!("unimplemented"),
        Some("rm") => unimplemented!("unimplemented"),
        _ => panic!(),
    }
}
