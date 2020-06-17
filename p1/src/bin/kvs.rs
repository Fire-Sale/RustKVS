// #[allow(dead_code)]
use clap::{App, Arg, SubCommand};
use std::process::exit;

fn main() {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("set")
                .about("Set <k, v> pair")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get <k, v> pair")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove <k, v> pair")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", _) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", _) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", _) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => panic!(),
    }
}
