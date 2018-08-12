use url::Url;

use super::fritzing::exporter::Exporter;
use super::fritzing::generator::Generator;
use failure::Error;
use projects::project::Project;

/// Manages access to the schematic related information.
#[derive(Clone)]
pub struct SchematicProvider {
    /// Instance of the exporter that is responsible for converting schematic into set of SVG files.
    exporter: Exporter,
    /// Instance of the generator that is responsible for generating schematic from the project.
    generator: Generator,
}

impl SchematicProvider {
    /// Returns a ready schematic provider instance.
    ///
    /// # Arguments
    ///
    /// * `schematic_base_url` - Base portion of the URL pointing to generated schematic sketches.
    /// * `export_api_url` - URL of the Export API that will do the sketch-to-svg export.
    /// * `generated_content_folder` - Path on the disk to store generated content to.
    pub fn new<T: Into<String>>(
        schematic_base_url: Url,
        export_api_url: Url,
        generated_content_folder: T,
    ) -> Self {
        SchematicProvider {
            exporter: Exporter::new(schematic_base_url, export_api_url),
            generator: Generator::new(generated_content_folder),
        }
    }

    /// Gets schematic (byte array for the schematic image) for the project.
    ///
    /// # Arguments
    ///
    /// * `project` - Project to get schematic for.
    pub fn get(&self, project: Project) -> Result<Vec<u8>, Error> {
        let path = self.generator.generate_sketch(project)?;

        info!("Sketch generated and stored at {:?}", path);

        self.exporter.export_sketch_to_svg(&path)
    }
}
