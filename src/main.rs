#![cfg_attr(feature = "stainless", feature(plugin))]
#![cfg_attr(test, plugin(stainless))]

extern crate docopt;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate log;
extern crate router;
extern crate rustc_serialize;

use docopt::Docopt;

use iron::prelude::*;
use iron::status;
use router::Router;

const USAGE: &'static str = "
Usage: frunze_api [--verbose] [--ip=<address>] [--port=<port>]
       frunze_api --help
Options:
    --ip <ip>       IP (v4) address to listen on [default: 0.0.0.0].
    --port <port>   Port number to listen on [default: 8009].
    --verbose       Toggle verbose output.
    --help          Print this help menu.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_ip: Option<String>,
    flag_port: Option<u16>,
    flag_verbose: bool,
    flag_help: bool,
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");

    let ip = args.flag_ip.unwrap_or("0.0.0.0".to_owned());
    let port = args.flag_port.unwrap_or(8009);

    info!("Running server at {}:{}", ip, port);

    Iron::new(router).http((ip.as_ref(), port)).unwrap();
}


#[cfg(test)]
describe! main {
    describe! args {
        it "should have default values" {
            let args: super::super::Args = super::super::Docopt::new(USAGE)
                .and_then(|d| d.decode())
                .unwrap_or_else(|e| e.exit());

            assert_eq!(args.flag_verbose, false);
            assert_eq!(args.flag_ip, None);
            assert_eq!(args.flag_port, None);
            assert_eq!(args.flag_help, false);
        }
    }
}
