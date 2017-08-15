use std::net::SocketAddr;
use std::path::PathBuf;
use url::Url;
use errors::{Error, Result};
use futures::{future, Future, Stream};
use hyper::{Error as HyperError, Client};
use tokio_core::reactor::Core;

#[derive(Clone)]
pub struct Exporter {
    host_address: SocketAddr,
    api_url: String,
}

impl Exporter {
    pub fn new<T: Into<String>>(host_address: SocketAddr, api_url: T) -> Self {
        Exporter {
            host_address: host_address,
            api_url: api_url.into(),
        }
    }

    pub fn export_sketch_to_svg(&self, sketch_path: PathBuf) -> Result<Vec<u8>> {
        // We control file name (it's UUID) so it should be a valid Unicode string.
        let file_name = sketch_path.file_name().unwrap().to_string_lossy();

        let url = Url::parse(&format!(
            "{}/svg-tcp/http://{}/schematic/generated/{}",
            &self.api_url,
            self.host_address,
            file_name
        ))?;

        info!("URL: {}", url);

        let mut core = Core::new()?;
        let client = Client::new(&core.handle());

        let work = client.get(url.into_string().parse()?).and_then(|res| {
            res.body().fold(Vec::new(), |mut v: Vec<u8>, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, HyperError>(v)
            })
        });

        core.run(work).map_err(|e| -> Error { e.into() })
    }
}
