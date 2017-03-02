#![cfg_attr(feature = "stainless", feature(plugin))]
#![cfg_attr(test, plugin(stainless))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![recursion_limit = "1024"]

extern crate bson;
extern crate mongodb;

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
#[macro_use]
extern crate error_chain;

mod errors;
mod core;
mod db;

use db::DB;

use docopt::Docopt;

use iron::method::Method;
use iron::headers;
use iron::mime;
use iron::prelude::*;
use iron::status;
use router::Router;
use unicase::UniCase;
use errors::*;

const USAGE: &'static str = "
Usage: frunze_api [--verbose] [--ip=<address>] [--port=<port>] [--db-ip=<address>] [--db-port=<port>] [--db-name=<name>]
       frunze_api --help
Options:
    --ip <ip>           IP (v4) address to listen on [default: 0.0.0.0].
    --port <port>       Port number to listen on [default: 8009].
    --db-ip <ip>        IP (v4) address of the database [default: 0.0.0.0].
    --db-port <port>    Port number of the database [default: 27017].
    --db-name <name>    Name of the database to use [default: frunze].
    --verbose           Toggle verbose output.
    --help              Print this help menu.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_ip: Option<String>,
    flag_port: Option<u16>,
    flag_db_ip: Option<String>,
    flag_db_port: Option<u16>,
    flag_db_name: Option<String>,
    flag_verbose: bool,
    flag_help: bool,
}

fn add_cors_headers(response: &mut Response) {
    response.headers.set(headers::AccessControlAllowOrigin::Any);
    response.headers.set(headers::AccessControlAllowHeaders(
        vec![UniCase(String::from("accept")), UniCase("content-type".to_string())]
    ));
    response.headers.set(headers::AccessControlAllowMethods(vec![Method::Get,
                                                                 Method::Post,
                                                                 Method::Put,
                                                                 Method::Delete]));
}

fn handler(request: &mut Request, database: &DB) -> IronResult<Response> {
    info!("Request received: {}", request.url);
    /*let query_option = req.extensions.get::<Router>().unwrap().find("query");*/
    let content_type = mime::Mime(mime::TopLevel::Application, mime::SubLevel::Json,
                                  vec![(mime::Attr::Charset, mime::Value::Utf8)]);
    let response_body = serde_json::to_string(&database.get_control_groups()?)
        .map_err(|e| -> Error { e.into() })?;

    let mut response = Response::with((content_type, status::Ok, response_body));

    // Add required CORS headers, we should be more strict here in production obviously.
    add_cors_headers(&mut response);

    Ok(response)
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let db_ip = args.flag_db_ip.unwrap_or_else(|| "0.0.0.0".to_string());
    let db_port = args.flag_db_port.unwrap_or(27017);
    let db_name = args.flag_db_name.unwrap_or_else(|| "frunze".to_string());

    info!("Connecting to the database `{}` at {}:{}", db_name, db_ip, db_port);
    let mut database = DB::new(db_name);
    database.connect(&db_ip, db_port).expect("Failed to connect to the database.");

    let mut router = Router::new();
    router.get("/control-groups", move |request: &mut Request| handler(request, &database),
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
            assert_eq!(args.flag_db_ip, None);
            assert_eq!(args.flag_db_port, None);
            assert_eq!(args.flag_db_name, None);
            assert_eq!(args.flag_help, false);
        }
    }
}
