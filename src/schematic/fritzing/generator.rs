use std::path::PathBuf;

use errors::Result;
use projects::project::Project;

/// Generator is responsible for generation of Fritzing sketches based on the `Project`, its
/// components and defined relationships between them.
#[derive(Clone)]
pub struct Generator {
    /// Path on the disk to store generated content to.
    output_folder_path: String,
}

impl Generator {
    /// Returns a ready generator instance.
    ///
    /// # Arguments
    ///
    /// * `output_folder_path` - Path on the disk to store generated content to.
    pub fn new<T: Into<String>>(output_folder_path: T) -> Self {
        Generator { output_folder_path: output_folder_path.into() }
    }

    /// Takes project, extracts all the components and builds Fritzing sketch based on this data.
    /// Then sketch is saved into configured folder and path to that file is returned.
    ///
    /// # Arguments
    ///
    /// * `project` - Project instance to generate sketch for.
    pub fn generate_sketch(&self, project: Project) -> Result<PathBuf> {
        // TODO: Just temporarily rely on existing files and implement actual generation later.
        Ok(
            PathBuf::from(&self.output_folder_path)
                .with_file_name(project.id)
                .with_extension("fzz"),
        )
    }
}
