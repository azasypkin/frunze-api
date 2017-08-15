use std;
use std::io::prelude::*;
use std::io::Cursor;
use std::net::SocketAddr;
use std::path::PathBuf;
use url::Url;
use errors::{Error, Result};
use futures::{future, Future, Stream};
use hyper::{Error as HyperError, Client};
use tokio_core::reactor::Core;
use zip;

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
        let project_id = sketch_path.file_stem().unwrap().to_string_lossy();

        let reader = Cursor::new(self.download_export_archive(&file_name)?);

        let mut zip = zip::ZipArchive::new(reader)?;
        let breadboard_image = zip.by_name(&format!("{}_breadboard.svg", project_id))?;

        Ok(breadboard_image.bytes().collect::<std::io::Result<Vec<u8>>>()?)
    }

    fn download_export_archive(&self, file_name: &str) -> Result<Vec<u8>> {
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
