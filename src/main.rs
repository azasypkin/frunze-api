#![cfg_attr(feature = "stainless", feature(plugin))]
#![cfg_attr(test, plugin(stainless))]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

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
extern crate serde_json;
extern crate unicase;

mod core;

use docopt::Docopt;

use iron::method::Method;
use iron::headers;
use iron::prelude::*;
use iron::status;
use router::Router;
use unicase::UniCase;

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
    let query_option = req.extensions
        .get::<Router>()
        .unwrap()
        .find("query");

    Ok(Response::with((status::Ok, query_option.unwrap_or("/"))))
}

fn add_cors_headers(response: &mut Response) {
    response.headers.set(headers::AccessControlAllowOrigin::Any);
    response.headers.set(headers::AccessControlAllowHeaders(
        vec![UniCase(String::from("accept")), UniCase(String::from("content-type"))]
    ));
    response.headers.set(headers::AccessControlAllowMethods(vec![Method::Get,
                                                                 Method::Post,
                                                                 Method::Put,
                                                                 Method::Delete]));
}

fn get_control_groups_handler(_: &mut Request) -> IronResult<Response> {
    let content_type = "application/json".parse::<iron::mime::Mime>().unwrap();

    let fake_response = vec![core::control_group::ControlGroup {
                                 type_name: "group#1".to_string(),
                                 name: "Group #1".to_string(),
                                 description: "Group #1 Description".to_string(),
                                 items: vec![core::control_metadata::ControlMetadata {
                                                 type_name: "type#11".to_string(),
                                                 name: "Item #11".to_string(),
                                                 description: "Item #11 Description".to_string(),
                                             },
                                             core::control_metadata::ControlMetadata {
                                                 type_name: "type#12".to_string(),
                                                 name: "Item #12".to_string(),
                                                 description: "Item #12 Description".to_string(),
                                             }],
                             },
                             core::control_group::ControlGroup {
                                 type_name: "group#2".to_string(),
                                 name: "Group #2".to_string(),
                                 description: "Group #2 Description".to_string(),
                                 items: vec![core::control_metadata::ControlMetadata {
                                                 type_name: "type#21".to_string(),
                                                 name: "Item #21".to_string(),
                                                 description: "Item #21 Description".to_string(),
                                             },
                                             core::control_metadata::ControlMetadata {
                                                 type_name: "type#22".to_string(),
                                                 name: "Item #22".to_string(),
                                                 description: "Item #22 Description".to_string(),
                                             }],
                             }];

    let mut response =
        Response::with((content_type, status::Ok, serde_json::to_string(&fake_response).unwrap()));

    // Add required CORS headers, we should be more strict here in production obviously.
    add_cors_headers(&mut response);

    Ok(response)
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "query");
    router.get("/control-groups",
               get_control_groups_handler,
               "control-groups");

    let ip = args.flag_ip.unwrap_or_else(|| "0.0.0.0".to_string());
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
