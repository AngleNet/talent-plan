use clap::{App, Arg};
use std::fmt;

fn main() {
    bar();
}

// error handling

#[derive(Debug)]
enum MyErr {
    Reason1(String),
    Reason2(String, u32),
}

impl fmt::Display for MyErr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyErr::Reason1(ref s) => write!(fmt, "MyErr<{}>", s),
            MyErr::Reason2(ref s, ref u) => write!(fmt, "MyErr<{}, {}>", s, u),
        }
    }
}

fn foo(n: Option<usize>) -> Result<(), MyErr> {
    match n {
        Some(_) => Ok(()),
        None => Err(MyErr::Reason1("reason1".to_string())),
    }
}

fn bar() {
    match foo(None) {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
}

// clap argument processing

#[allow(dead_code)]
fn test_clap() {
    let app = App::new("myapp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::new("INPUT")
                .index(1)
                .required(true)
                .about("Sets the input file to use"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .multiple(true)
                .about("Sets the level of verbosity"),
        )
        .subcommand(
            App::new("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(
                    Arg::new("debug")
                        .short('d')
                        .about("pritn debug information"),
                ),
        );
    let _m = app.get_matches();
}
