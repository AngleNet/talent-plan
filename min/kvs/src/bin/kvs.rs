use clap::{App, Arg, SubCommand};
use std::process;
fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of the specified key")
                .arg(
                    Arg::with_name("KEY")
                        .required(true)
                        .help("Sets the specified key")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a key/value pair")
                .arg(
                    Arg::with_name("KEY")
                        .help("Sets the specified key")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VALUE")
                        .help("Sets the specified value")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Removes a key/value pair")
                .arg(
                    Arg::with_name("KEY")
                        .help("Sets the specified key")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    match matches.subcommand() {
        ("get", Some(cmd)) => unimplemented!("get"),
        ("rm", Some(cmd)) => unimplemented!("rm"),
        ("set", Some(cmd)) => unimplemented!("set"),
        _ => {}
    }
    process::exit(1);
}
