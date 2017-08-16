use std;
use std::io::prelude::*;
use std::io::Cursor;
use std::path::PathBuf;

use futures::{future, Future, Stream};
use hyper::{Error as HyperError, Client};
use tokio_core::reactor::Core;
use url::Url;
use zip;

use errors::{Error, Result};

/// Exporter is responsible for exporting of Fritzing sketches to a list of SVG files.
#[derive(Clone)]
pub struct Exporter {
    /// Base portion of the URL pointing to generated schematic sketches. It's used to
    /// construct full schematic download URL used by Fritzing server.
    schematic_base_url: Url,
    /// URL of the Fritzing server that will do the sketch-to-svg export.
    api_url: Url,
}

impl Exporter {
    /// Returns a ready exporter instance.
    ///
    /// # Arguments
    ///
    /// * `schematic_base_url` - Base portion of the URL pointing to generated schematic sketches.
    /// * `api_url` - Address of the Fritzing server.
    pub fn new(schematic_base_url: Url, api_url: Url) -> Self {
        Exporter {
            schematic_base_url: schematic_base_url,
            api_url: api_url,
        }
    }

    pub fn export_sketch_to_svg(&self, sketch_path: PathBuf) -> Result<Vec<u8>> {
        // We control file name (it's UUID) so it should be a valid Unicode string.
        let file_name = sketch_path.file_name().unwrap().to_string_lossy();
        let project_id = sketch_path.file_stem().unwrap().to_string_lossy();

        let reader = Cursor::new(self.download_export_archive(&file_name)?);

        let mut zip = zip::ZipArchive::new(reader)?;
        let breadboard_image = zip.by_name(&format!("{}_breadboard.svg", project_id))?;

        Ok(breadboard_image
            .bytes()
            .collect::<std::io::Result<Vec<u8>>>()?)
    }

    fn download_export_archive(&self, file_name: &str) -> Result<Vec<u8>> {
        let url = Url::parse(&format!(
            "{}/svg-tcp/{}{}",
            &self.api_url.as_str(),
            self.schematic_base_url.as_str(),
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
