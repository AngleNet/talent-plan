use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;
use structopt::StructOpt;
fn main() {
    parse_cmd_via_opt();
    process::exit(1);
}

#[derive(Debug, StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"),
    about=env!("CARGO_PKG_DESCRIPTION"), author=env!("CARGO_PKG_AUTHORS"),
    version=env!("CARGO_PKG_VERSION"))]
struct Options {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "get", about = "Get the value")]
    Get {
        #[structopt(help = "A string key", name = "KEY")]
        key: String,
    },
    #[structopt(name = "set", about = "Set a key value pair")]
    Set {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(name = "VALUE", help = "A string value")]
        value: String,
    },
    #[structopt(name = "rm", about = "Remove a key value pair")]
    Remove {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
    },
}

fn parse_cmd_via_opt() {
    let opts = Options::from_args();
    println!("{:?}", opts);
}

fn parse_cmd_via_clap() {
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
        ("get", Some(_cmd)) => unimplemented!("get"),
        ("rm", Some(_cmd)) => unimplemented!("rm"),
        ("set", Some(_cmd)) => unimplemented!("set"),
        _ => {}
    }
}
