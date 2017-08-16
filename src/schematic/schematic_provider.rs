use url::Url;
use errors::Result;

use projects::project::Project;
use super::fritzing::generator::Generator;
use super::fritzing::exporter::Exporter;

/// Manages access to the schematic related information.
#[derive(Clone)]
pub struct SchematicProvider {
    exporter: Exporter,
    generator: Generator,
}

impl SchematicProvider {
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
    pub fn get(&self, project: Project) -> Result<Vec<u8>> {
        let path = self.generator.generate_sketch(project)?;

        info!("Sketch generated: {:?}", path);

        self.exporter.export_sketch_to_svg(path)
    }
}
