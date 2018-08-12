#![cfg_attr(feature = "stainless", feature(plugin))]
#![cfg_attr(test, plugin(stainless))]

extern crate actix_web;
extern crate bytes;
#[macro_use(bson, doc)]
extern crate bson;
extern crate docopt;
extern crate env_logger;
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate mongodb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate uuid;
extern crate zip;

use bytes::Bytes;
use std::path::PathBuf;

use actix_web::{
    fs::NamedFile, http, middleware::cors::Cors, middleware::cors::CorsBuilder, server, App,
    HttpRequest, HttpResponse, Json, Result, State,
};
use std::net::{IpAddr, SocketAddr};

mod bom;
mod components;
mod db;
mod projects;
mod schematic;

use bom::bom_provider::BomProvider;
use db::DB;
use docopt::Docopt;
use failure::Error;
use schematic::schematic_provider::SchematicProvider;

use url::Url;

const USAGE: &str = "
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

struct AppState {
    database: DB,
    bom_provider: BomProvider,
    schematic_provider: SchematicProvider,
}

fn json_handler<F, T: Sized>(
    request: &HttpRequest<AppState>,
    content_retriever: F,
) -> Result<Json<T>>
where
    F: FnOnce() -> Result<T, Error>,
    T: serde::Serialize,
{
    info!("Request received: {}", request.path());

    Ok(Json(content_retriever()?))
}

fn setup_db_routes(app: &mut CorsBuilder<AppState>) {
    app.resource("/component-groups", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            json_handler(req, || req.state().database.get_component_groups())
        })
    }).resource("/component-schemas", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            json_handler(req, || req.state().database.get_component_schemas())
        })
    }).resource("/projects", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            json_handler(req, || req.state().database.get_projects())
        })
    }).resource("/project-capabilities", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            json_handler(req, || req.state().database.get_project_capabilities())
        })
    }).resource("/project-capability-groups", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            json_handler(req, || req.state().database.get_project_capability_groups())
        })
    }).resource("/project-platforms", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            json_handler(req, || req.state().database.get_project_platforms())
        })
    }).resource("/project", move |r| {
        // FIXME: Right now we have only POST method implemented, but later on this method should be
        // used only for new projects, if we receive project with non-empty ID we should throw and
        // recommend using PUT method, similar behavior should be implemented for PUT method.
        r.method(http::Method::POST).with(
            |data: (State<AppState>, Json<projects::project::Project>)| {
                let (state, project_to_save) = data;
                state
                    .database
                    .save_project(project_to_save.into_inner())
                    .map(|project| HttpResponse::Ok().body(project.id))
                    .unwrap_or_else(|_| HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR))
            },
        )
    }).resource("/project/{id}", move |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            let project_id: String = req.match_info().query("id")?;
            json_handler(req, || req.state().database.get_project(&project_id))
        });

        r.delete().f(|req: &HttpRequest<AppState>| {
            let project_id: String = req.match_info().query("id")?;
            json_handler(req, || req.state().database.delete_project(&project_id))
        })
    });
}

fn setup_bom_routes(app: &mut CorsBuilder<AppState>) {
    app.resource("/bom/part/{uid}", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            let part_uid: String = req.match_info().query("uid")?;
            json_handler(req, || req.state().bom_provider.get_part(part_uid))
        })
    }).resource("/bom/parts/{mpn}", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            let mpn: String = req.match_info().query("mpn")?;
            json_handler(req, || {
                req.state()
                    .bom_provider
                    .find_parts(mpn.split(',').collect())
            })
        })
    });
}

fn setup_schematic_routes(app: &mut CorsBuilder<AppState>) {
    app.resource("/schematic/{id}", |r| {
        r.get().f(|req: &HttpRequest<AppState>| {
            req.match_info()
                .query::<String>("id")
                .map_err(|_| actix_web::error::ErrorBadRequest("Bad project id"))
                .and_then(|project_id| {
                    req.state()
                        .database
                        .get_project(&project_id)
                        .map_err(|err| actix_web::error::ErrorInternalServerError(err))
                        .and_then(|project| {
                            project.ok_or_else(|| {
                                info!("Project with id {} not found", project_id);
                                actix_web::error::ErrorNotFound(format!(
                                    "Project with id {} not found",
                                    project_id
                                ))
                            })
                        })
                }).and_then(|project| {
                    req.state()
                        .schematic_provider
                        .get(project)
                        .map_err(|err| actix_web::error::ErrorInternalServerError(err))
                }).and_then(|data| {
                    Ok(HttpResponse::Ok()
                        .content_type("image/svg+xml")
                        .body(Bytes::from(data)))
                })
        })
    });
}

fn main() {
    env_logger::init();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let ip = args.flag_ip.unwrap_or_else(|| "0.0.0.0".to_string());
    let port = args.flag_port.unwrap_or(8009);
    let host_address: SocketAddr = SocketAddr::new(IpAddr::V4(ip.parse().unwrap()), port);

    let bom_api_url = args
        .flag_bom_api_url
        .or_else(|| Some("http://octopart.com/api/v3".to_string()))
        .and_then(|url_string| Url::parse(url_string.as_ref()).ok())
        .unwrap();

    let export_api_url = args
        .flag_export_api_url
        .or_else(|| Some("http://localhost:8010".to_string()))
        .and_then(|url_string| Url::parse(url_string.as_ref()).ok())
        .unwrap();

    info!("BOM API is assigned to {}.", bom_api_url);
    info!("Export API is assigned to {}.", export_api_url);

    let db_ip = args.flag_db_ip.unwrap_or_else(|| "0.0.0.0".to_string());
    let db_port = args.flag_db_port.unwrap_or(27_017);
    let db_name = args.flag_db_name.unwrap_or_else(|| "frunze".to_string());
    let bom_api_key = args.flag_bom_api_key;

    info!(
        "Connecting to the database `{}` at {}:{}...",
        db_name, db_ip, db_port
    );

    info!("Running server at {}", host_address);

    server::new(move || {
        let mut database = DB::new(db_name.clone());
        database
            .connect(db_ip.as_ref(), db_port)
            .expect("Failed to connect to the database.");

        let bom_provider = BomProvider::new(bom_api_url.clone(), bom_api_key.clone());

        let generated_schematic_fragment = "/schematic/generated/";

        let schematic_provider = SchematicProvider::new(
            Url::parse(
                format!(
                    "http://{}:{}{}",
                    host_address.ip(),
                    host_address.port(),
                    generated_schematic_fragment
                ).as_ref(),
            ).unwrap(),
            export_api_url.clone(),
            "generated/schematic".to_string(),
        );

        let mut app = Cors::for_app(App::with_state(AppState {
            database,
            bom_provider,
            schematic_provider,
        }));

        app.allowed_origin("http://localhost:4200")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ]);

        setup_db_routes(&mut app);
        setup_bom_routes(&mut app);
        setup_schematic_routes(&mut app);

        // Serve generated schematic files. File name is "{project-id}.fzz".
        app.resource(r"/schematic/generated/{tail:.*}", |r| {
            r.get()
                .f(|req: &HttpRequest<AppState>| -> Result<NamedFile> {
                    let mut path: PathBuf = PathBuf::from("generated/schematic");
                    path.push(req.match_info().query::<String>("tail")?);

                    info!("Serving static file: {:?}", path);
                    Ok(NamedFile::open(path)?)
                })
        });

        app.register()
    }).bind(host_address)
    .unwrap()
    .run();
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
