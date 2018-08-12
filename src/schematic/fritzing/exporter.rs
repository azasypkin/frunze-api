use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use actix_web::{client, HttpMessage};
use failure::Error;
use futures::Future;
use url::Url;
use zip;

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
            schematic_base_url,
            api_url,
        }
    }

    /// Exports Fritzing sketch into SVG file (only breadboard view is extracted right now).
    ///
    /// # Arguments
    ///
    /// * `sketch_path` - Path to the generated sketch file.
    pub fn export_sketch_to_svg(&self, sketch_path: &PathBuf) -> Result<Vec<u8>, Error> {
        // We control file name (it's UUID) so it should be a valid Unicode string.
        let file_name = sketch_path.file_name().unwrap().to_string_lossy();
        let file_stem = sketch_path.file_stem().unwrap().to_string_lossy();

        let downloaded_archive = self.download_export_archive(&file_name)?;

        let mut zip_archive = zip::ZipArchive::new(io::Cursor::new(downloaded_archive))?;

        let mut breadboard_image = zip_archive.by_name(&format!("{}_breadboard.svg", file_stem))?;
        let mut buffer = Vec::new();
        breadboard_image.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    /// Downloads archive with exported SVG files from the Fritzing server.
    ///
    /// # Arguments
    ///
    /// * `file_name` - name of the Fritzing sketch file to extract SVG from.
    fn download_export_archive(&self, file_name: &str) -> Result<Vec<u8>, Error> {
        let archive_url = format!(
            "{}/svg-tcp/{}{}",
            &self.api_url.as_str(),
            self.schematic_base_url.as_str(),
            file_name
        );

        info!("Downloading archive from {}", archive_url);

        client::get(archive_url)
            .finish()
            .unwrap()
            .send()
            .map_err(|err| err.into())
            .and_then(|res| res.body().from_err().and_then(|bytes| Ok(bytes.to_vec())))
            .wait()
    }
}
