#![cfg_attr(feature = "stainless", feature(plugin))]
#![cfg_attr(test, plugin(stainless))]

extern crate docopt;
extern crate env_logger;
extern crate iron;
#[macro_use]
extern crate log;
extern crate router;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod core;

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

fn get_control_groups_handler(_: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();

    let fake_response = vec![core::control_group::ControlGroup {
        type_name: "group#1".to_owned(),
        name: "Group #1".to_owned(),
        description: "Group #1 Description".to_owned(),
        items: vec![core::control_metadata::ControlMetadata {
            type_name: "type#11".to_owned(),
            name: "Item #11".to_owned(),
            description: "Item #11 Description".to_owned(),
        }, core::control_metadata::ControlMetadata {
            type_name: "type#12".to_owned(),
            name: "Item #12".to_owned(),
            description: "Item #12 Description".to_owned(),
        }]
    }, core::control_group::ControlGroup {
        type_name: "group#2".to_owned(),
        name: "Group #2".to_owned(),
        description: "Group #2 Description".to_owned(),
        items: vec![core::control_metadata::ControlMetadata {
            type_name: "type#21".to_owned(),
            name: "Item #21".to_owned(),
            description: "Item #21 Description".to_owned(),
        }, core::control_metadata::ControlMetadata {
            type_name: "type#22".to_owned(),
            name: "Item #22".to_owned(),
            description: "Item #22 Description".to_owned(),
        }]
    }];

    Ok(Response::with((content_type, status::Ok, serde_json::to_string(&fake_response).unwrap())))
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");
    router.get("/control-groups", get_control_groups_handler, "control-groups");

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
