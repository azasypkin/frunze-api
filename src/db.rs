use bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use errors::*;
use serde;

use editor::component_group::ComponentGroup;
use projects::project::Project;
use projects::project_capability_group::ProjectCapabilityGroup;
use projects::project_capability::ProjectCapability;
use projects::project_platform::ProjectPlatform;

#[derive(Clone)]
pub struct DB {
    name: String,
    client: Option<Client>,
}

impl DB {
    pub fn new<T: Into<String>>(name: T) -> Self {
        DB {
            name: name.into(),
            client: None,
        }
    }

    pub fn connect(&mut self, host: &str, port: u16) -> Result<()> {
        self.client = Some(Client::connect(host, port)?);
        Ok(())
    }

    /// Queries project instance from the database using passed `project_id`.
    pub fn get_project(&self, project_id: &str) -> Result<Option<Project>> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let result = db.collection("projects").find_one(
            Some(doc! { "id" => project_id }),
            None,
        )?;
        let result = if let Some(project) = result {
            Some(bson::from_bson(bson::Bson::Document(project))?)
        } else {
            None
        };

        Ok(result)
    }

    /// Saves project to the database.
    pub fn save_project(&self, project: &Project) -> Result<()> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let collection = db.collection("projects");
        if let bson::Bson::Document(document) = bson::to_bson(&project)? {
            collection.insert_one(document, None)?;
        }

        Ok(())
    }

    /// Queries component groups from the database.
    pub fn get_component_groups(&self) -> Result<Vec<ComponentGroup>> {
        self.get_collection("component_groups")
    }

    /// Queries project capability groups from the database.
    pub fn get_project_capability_groups(&self) -> Result<Vec<ProjectCapabilityGroup>> {
        self.get_collection("project_capability_groups")
    }

    /// Queries all known project capabilities from the database.
    pub fn get_project_capabilities(&self) -> Result<Vec<ProjectCapability>> {
        self.get_collection("project_capabilities")
    }

    /// Queries all known project platforms from the database.
    pub fn get_project_platforms(&self) -> Result<Vec<ProjectPlatform>> {
        self.get_collection("project_platforms")
    }

    fn get_collection<T>(&self, collection_name: &str) -> Result<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let mut result: Vec<T> = vec![];
        let cursor = db.collection(collection_name).find(None, None)?;
        for cursor_item in cursor {
            info!("Iterating through database records {:?}", cursor_item);
            if let Ok(item) = cursor_item {
                result.push(bson::from_bson(bson::Bson::Document(item))?);
            }
        }

        Ok(result)
    }
}
