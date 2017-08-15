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
extern crate staticfile;
extern crate unicase;
extern crate url;
extern crate uuid;
extern crate zip;

use std::net::{SocketAddr, IpAddr};
use std::path::Path;

mod middleware;
mod errors;
mod components;
mod projects;
mod db;
mod bom;
mod schematic;

use std::io::Read;
use db::DB;

use docopt::Docopt;

use iron::{Chain, mime, status};
use iron::prelude::*;
use mount::Mount;
use router::Router;
use staticfile::Static;
use errors::{Error, ErrorKind, Result};
use bom::bom_provider::BomProvider;
use schematic::schematic_provider::SchematicProvider;

use url::percent_encoding::percent_decode;

const USAGE: &'static str = "
Usage: frunze_api [--verbose] [--ip=<address>] [--port=<port>] [--db-ip=<address>]
                  [--db-port=<port>] [--db-name=<name>] [--bom-api-url=<url>]
                  [--bom-api-key=<key>] [--export-api-url=<url>]
       frunze_api --help
Options:
    --ip <ip>               IP (v4) address to listen on [default: 0.0.0.0].
    --port <port>           Port number to listen on [default: 8009].
    --db-ip <ip>            IP (v4) address of the database [default: 0.0.0.0].
    --db-port <port>        Port number of the database [default: 27017].
    --db-name <name>        Name of the database to use [default: frunze].
    --bom-api-url <url>     URL of BOM API provider [default: http://octopart.com/api/v3].
    --bom-api-key <key>     API key to use for all requests to BOM API provider.
    --export-api-url <url>  URL of Schematic Export API provider [default: http://localhost:8010].
    --verbose               Toggle verbose output.
    --help                  Print this help menu.
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
    flag_export_api_url: Option<String>,
    flag_verbose: bool,
    flag_help: bool,
}

fn json_handler<F, T: Sized>(request: &Request, content_retriever: F) -> IronResult<Response>
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

fn get_router_argument(request: &Request, argument_name: &str) -> Result<String> {
    request
        .extensions
        .get::<Router>()
        .and_then(|router| router.find(argument_name))
        .and_then(|arg| percent_decode(arg.as_bytes()).decode_utf8().ok())
        .map(|arg| arg.to_string())
        .ok_or_else(|| {
            ErrorKind::RouterArgumentIsNotProvided(argument_name.to_string()).into()
        })
}

fn setup_db_routes(router: &mut Router, database: &DB) {
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
            let project_id = itry!(get_router_argument(request, "id"), status::BadRequest);
            json_handler(request, || db.get_project(&project_id))
        },
        "project",
    );

    let db = database.clone();
    router.delete(
        "/project/:id",
        move |request: &mut Request| {
            let project_id = itry!(get_router_argument(request, "id"), status::BadRequest);
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

fn setup_bom_routes(router: &mut Router, bom_provider: &BomProvider) {
    let bom = bom_provider.clone();
    router.get(
        "/bom/part/:uid",
        move |request: &mut Request| {
            let part_uid = itry!(get_router_argument(request, "uid"), status::BadRequest);
            json_handler(request, || bom.get_part(part_uid))
        },
        "bom-part",
    );

    let bom = bom_provider.clone();
    router.get(
        "/bom/parts/:mpn",
        move |request: &mut Request| {
            let mpn = itry!(get_router_argument(request, "mpn"), status::BadRequest);
            json_handler(request, || bom.find_parts(mpn.split(",").collect()))
        },
        "bom-find-parts",
    );
}

fn setup_schematic_routes(
    router: &mut Router,
    schematic_provider: &SchematicProvider,
    database: &DB,
) {
    let schematic_provider = schematic_provider.clone();
    let db = database.clone();
    router.get(
        "/schematic/:project-id",
        move |request: &mut Request| {
            let project_id = itry!(
                get_router_argument(request, "project-id"),
                status::BadRequest
            );
            let project = db.get_project(&project_id)?;

            let project = project.ok_or_else(|| {
                let err: Error = ErrorKind::ProjectNotFound(project_id.to_string()).into();
                iron::IronError::new(err, status::NotFound)
            })?;

            let content = schematic_provider.get(project)?;
            let content_type = "image/svg+xml".parse::<mime::Mime>().unwrap();
            Ok(Response::with((content_type, status::Ok, content)))
        },
        "schematic-project",
    );
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let ip = args.flag_ip.unwrap_or_else(|| "0.0.0.0".to_string());
    let port = args.flag_port.unwrap_or(8009);
    let host_address: SocketAddr = SocketAddr::new(IpAddr::V4(ip.parse().unwrap()), port);

    let bom_api_url = args.flag_bom_api_url.unwrap_or_else(|| {
        "http://octopart.com/api/v3".to_string()
    });

    let export_api_url = args.flag_export_api_url.unwrap_or_else(|| {
        "http://localhost:8010".to_string()
    });

    info!("BoM API is assigned to {}.", bom_api_url);
    info!("export API is assigned to {}.", export_api_url);

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

    let schematic_provider = SchematicProvider::new(
        host_address,
        export_api_url,
        "generated/schematic".to_string(),
    );

    let mut router = Router::new();

    setup_db_routes(&mut router, &database);
    setup_bom_routes(&mut router, &bom_provider);
    setup_schematic_routes(&mut router, &schematic_provider, &database);

    let mut mount = Mount::new();
    mount.mount("/", router);

    // Serve generated schematic files. File name is "{project-id}.fzz".
    mount.mount(
        "/schematic/generated/",
        Static::new(Path::new("generated/schematic")),
    );

    let mut chain = Chain::new(mount);
    chain.link_after(middleware::cors::CORSMiddleware);

    info!("Running server at {}", host_address);
    Iron::new(chain).http(host_address).unwrap();
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
