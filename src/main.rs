extern crate rustc_serialize;
extern crate ansi_term;
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate log;

//use ansi_term::Colour::{Green, Red};

use docopt::Docopt;

const USAGE: &'static str = "
Usage: frunze_api [--verbose] [--address=<address>] [--port=<port>]
       frunze_api --help
Options:
    --address <address>     Local network address [default: 0.0.0.0].
    --port=<port>           Local network port [default: 10].
    --verbose               Toggle verbose output.
    --help                  Print this help menu.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_address: String,
    flag_port: u16,
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);
}
