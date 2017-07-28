#![cfg_attr(feature = "stainless", feature(plugin))]
#![cfg_attr(test, plugin(stainless))]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#![recursion_limit = "1024"]

#[macro_use(bson, doc)]
extern crate bson;
extern crate docopt;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
extern crate mongodb;
extern crate mount;
extern crate futures;
extern crate hyper;
extern crate tokio_core;
#[macro_use]
extern crate iron;
#[macro_use]
extern crate log;
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate unicase;
extern crate url;
extern crate uuid;

mod middleware;
mod errors;
mod components;
mod projects;
mod db;
mod bom;

use std::io::Read;
use db::DB;

use docopt::Docopt;

use iron::{Chain, mime, status};
use iron::prelude::*;
use mount::Mount;
use router::Router;
use errors::{Error, Result};
use bom::bom_provider::BomProvider;

const USAGE: &'static str = "
Usage: frunze_api [--verbose] [--ip=<address>] [--port=<port>] [--db-ip=<address>]
                  [--db-port=<port>] [--db-name=<name>] [--bom-api-url=<url>]
                  [--bom-api-key=<key>]
       frunze_api --help
Options:
    --ip <ip>           IP (v4) address to listen on [default: 0.0.0.0].
    --port <port>       Port number to listen on [default: 8009].
    --db-ip <ip>        IP (v4) address of the database [default: 0.0.0.0].
    --db-port <port>    Port number of the database [default: 27017].
    --db-name <name>    Name of the database to use [default: frunze].
    --bom-api-url <url> URL of BOM API provider [default: http://octopart.com/api/v3].
    --bom-api-key <key> API key to use for all requests to BOM API provider.
    --verbose           Toggle verbose output.
    --help              Print this help menu.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_ip: Option<String>,
    flag_port: Option<u16>,
    flag_db_ip: Option<String>,
    flag_db_port: Option<u16>,
    flag_db_name: Option<String>,
    flag_bom_api_url: Option<String>,
    flag_bom_api_key: String,
    flag_verbose: bool,
    flag_help: bool,
}

fn json_handler<F, T: Sized>(request: &mut Request, content_retriever: F) -> IronResult<Response>
where
    F: FnOnce() -> Result<T>,
    T: serde::Serialize,
{
    info!("Request received: {}", request.url);
    let content_type = mime::Mime(
        mime::TopLevel::Application,
        mime::SubLevel::Json,
        vec![(mime::Attr::Charset, mime::Value::Utf8)],
    );
    let response_body = serde_json::to_string(&content_retriever()?).map_err(
        |e| -> Error {
            e.into()
        },
    )?;

    Ok(Response::with((content_type, status::Ok, response_body)))
}

fn setup_db_routers(router: &mut Router, database: &DB) {
    let db = database.clone();
    router.get(
        "/component-groups",
        move |request: &mut Request| json_handler(request, || db.get_component_groups()),
        "component-groups",
    );

    let db = database.clone();
    router.get(
        "/component-schemas",
        move |request: &mut Request| json_handler(request, || db.get_component_schemas()),
        "component-schemas",
    );

    let db = database.clone();
    router.get(
        "/project/:id",
        move |request: &mut Request| {
            let project_id = request
                .extensions
                .get::<Router>()
                .unwrap()
                .find("id")
                .unwrap()
                .to_owned();
            json_handler(request, || db.get_project(&project_id))
        },
        "project",
    );

    let db = database.clone();
    router.delete(
        "/project/:id",
        move |request: &mut Request| {
            let project_id = request
                .extensions
                .get::<Router>()
                .unwrap()
                .find("id")
                .unwrap()
                .to_owned();
            json_handler(request, || db.delete_project(&project_id))
        },
        "project",
    );

    let db = database.clone();
    router.get(
        "/projects",
        move |request: &mut Request| json_handler(request, || db.get_projects()),
        "projects",
    );

    let db = database.clone();
    router.post("/project", move |request: &mut Request| -> IronResult<Response> {
        let mut payload = String::new();
        itry!(request.body.read_to_string(&mut payload));

        // FIXME: Right now we have only POST method implemented, but later on this method should be
        // used only for new projects, if we receive project with non-empty ID we should throw and
        // recommend using PUT method, similar behavior should be implemented for PUT method.
        let project = db.save_project(itry!(serde_json::from_str(&payload)))?;

        Ok(Response::with((status::Ok, project.id)))
    }, "project-set");

    let db = database.clone();
    router.get(
        "/project-capabilities",
        move |request: &mut Request| json_handler(request, || db.get_project_capabilities()),
        "project-capabilities",
    );

    let db = database.clone();
    router.get(
        "/project-capability-groups",
        move |request: &mut Request| json_handler(request, || db.get_project_capability_groups()),
        "project-capability-groups",
    );

    let db = database.clone();
    router.get(
        "/project-platforms",
        move |request: &mut Request| json_handler(request, || db.get_project_platforms()),
        "project-platforms",
    );
}

fn setup_bom_routers(router: &mut Router, bom_provider: &BomProvider) {
    let bom = bom_provider.clone();
    router.get(
        "/bom/part/:uid",
        move |request: &mut Request| {
            let part_uid = request
                .extensions
                .get::<Router>()
                .unwrap()
                .find("uid")
                .unwrap()
                .to_owned();
            json_handler(request, || bom.get_part(part_uid))
        },
        "bom-part",
    );

    let bom = bom_provider.clone();
    router.get(
        "/bom/parts/:mpn",
        move |request: &mut Request| {
            let mpn = request
                .extensions
                .get::<Router>()
                .unwrap()
                .find("mpn")
                .unwrap()
                .to_owned();
            json_handler(request, || bom.find_parts(mpn))
        },
        "bom-find-parts",
    );
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let bom_api_url = args.flag_bom_api_url.unwrap_or_else(|| {
        "http://octopart.com/api/v3".to_string()
    });

    info!("BoM API is assigned to {}.", bom_api_url);

    let bom_provider = BomProvider::new(bom_api_url, args.flag_bom_api_key);

    let db_ip = args.flag_db_ip.unwrap_or_else(|| "0.0.0.0".to_string());
    let db_port = args.flag_db_port.unwrap_or(27017);
    let db_name = args.flag_db_name.unwrap_or_else(|| "frunze".to_string());

    info!(
        "Connecting to the database `{}` at {}:{}...",
        db_name,
        db_ip,
        db_port
    );

    let mut database = DB::new(db_name);
    database.connect(&db_ip, db_port).expect(
        "Failed to connect to the database.",
    );

    let mut router = Router::new();

    setup_db_routers(&mut router, &database);
    setup_bom_routers(&mut router, &bom_provider);

    let mut mount = Mount::new();
    mount.mount("/", router);

    let mut chain = Chain::new(mount);
    chain.link_after(middleware::cors::CORSMiddleware);

    let ip = args.flag_ip.unwrap_or_else(|| "0.0.0.0".to_string());
    let port = args.flag_port.unwrap_or(8009);
    info!("Running server at {}:{}", ip, port);
    Iron::new(chain).http((ip.as_ref(), port)).unwrap();
}


#[cfg(test)]
describe! main {
    describe! args {
        it "should have default values" {
            let args: super::super::Args = super::super::Docopt::new(USAGE)
                .and_then(|d| d.deserialize())
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
