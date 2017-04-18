use bson;
use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use errors::*;

use editor::control_group::ControlGroup;
use editor::control_metadata::ControlMetadata;
use project::project_kind::ProjectKind;
use project::project_capability::ProjectCapability;

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

        // TODO: Move this to a docker initialization script.
        self.fill_db_if_empty()?;

        Ok(())
    }

    /// Queries control groups from the database.
    pub fn get_control_groups(&self) -> Result<Vec<ControlGroup>> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let mut result: Vec<ControlGroup> = vec![];
        let cursor = db.collection("groups").find(None, None)?;
        for cursor_item in cursor {
            info!("Iterating through database records {:?}", cursor_item);
            if let Ok(item) = cursor_item {
                result.push(bson::from_bson(bson::Bson::Document(item))?);
            }
        }

        Ok(result)
    }

    /// Queries project kinds from the database.
    pub fn get_project_kinds(&self) -> Result<Vec<ProjectKind>> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let mut result: Vec<ProjectKind> = vec![];
        let cursor = db.collection("project_kinds").find(None, None)?;
        for cursor_item in cursor {
            info!("Iterating through database records {:?}", cursor_item);
            if let Ok(item) = cursor_item {
                result.push(bson::from_bson(bson::Bson::Document(item))?);
            }
        }

        Ok(result)
    }

    /// Queries project capabilities from the database.
    pub fn get_project_capabilities(&self) -> Result<Vec<ProjectCapability>> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let mut result: Vec<ProjectCapability> = vec![];
        let cursor = db.collection("project_capabilities").find(None, None)?;
        for cursor_item in cursor {
            info!("Iterating through database records {:?}", cursor_item);
            if let Ok(item) = cursor_item {
                result.push(bson::from_bson(bson::Bson::Document(item))?);
            }
        }

        Ok(result)
    }

    fn fill_db_if_empty(&self) -> Result<()> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let groups_collection = db.collection("groups");
        if groups_collection.find_one(None, None)?.is_none() {
            info!("Control groups are not ready, filling DB with the corresponding data...");
            groups_collection.insert_many(self.generate_control_groups()?, None)?;
        }

        let project_kinds_collection = db.collection("project_kinds");
        if project_kinds_collection.find_one(None, None)?.is_none() {
            info!("Project kinds are not ready, filling DB with the corresponding data...");
            project_kinds_collection.insert_many(self.generate_project_kinds()?, None)?;
        }

        let project_capabilities_collection = db.collection("project_capabilities");
        if project_capabilities_collection.find_one(None, None)?.is_none() {
            info!("Project capabilities are not ready, filling DB with the corresponding data...");
            project_capabilities_collection.insert_many(self.generate_project_capabilities()?,
                                                        None)?;
        }

        info!("Database is ready.");

        Ok(())
    }

    // TODO: Below are temporary methods that will be replaced by the pre-loaded DB files.
    fn generate_control_groups(&self) -> Result<Vec<Document>> {
        let control_groups = vec![ControlGroup {
            type_name: "group#1".to_string(),
            name: "Group #1".to_string(),
            description: "Group #1 Description".to_string(),
            items: vec![ControlMetadata {
                type_name: "type#11".to_string(),
                name: "Item #11".to_string(),
                description: "Item #11 Description".to_string(),
            }, ControlMetadata {
                type_name: "type#12".to_string(),
                name: "Item #12".to_string(),
                description: "Item #12 Description".to_string(),
            }],
        }, ControlGroup {
            type_name: "group#2".to_string(),
            name: "Group #2".to_string(),
            description: "Group #2 Description".to_string(),
            items: vec![ControlMetadata {
                type_name: "type#21".to_string(),
                name: "Item #21".to_string(),
                description: "Item #21 Description".to_string(),
            }, ControlMetadata {
                type_name: "type#22".to_string(),
                name: "Item #22".to_string(),
                description: "Item #22 Description".to_string(),
            }],
        }];

        let bson_entities: Vec<Bson> = control_groups.iter()
            .map(|entity| bson::to_bson(&entity).map_err(|e| e.into()))
            .collect::<Result<Vec<Bson>>>()?;

        Ok(bson_entities.into_iter().filter_map(|entity| {
            if let Bson::Document(document) = entity { Some(document) } else { None }
        }).collect::<Vec<Document>>())
    }

    fn generate_project_kinds(&self) -> Result<Vec<Document>> {
        let project_kinds = vec![ProjectKind {
            type_name: "indicator".to_string(),
            name: "Indicator".to_string(),
        }, ProjectKind {
            type_name: "sensor".to_string(),
            name: "Sensor".to_string(),
        }, ProjectKind {
            type_name: "actuator".to_string(),
            name: "Actuator".to_string(),
        }, ProjectKind {
            type_name: "custom".to_string(),
            name: "Custom".to_string(),
        }];

        let bson_entities: Vec<Bson> = project_kinds.iter()
            .map(|entity| bson::to_bson(&entity).map_err(|e| e.into()))
            .collect::<Result<Vec<Bson>>>()?;

        Ok(bson_entities.into_iter().filter_map(|entity| {
            if let Bson::Document(document) = entity { Some(document) } else { None }
        }).collect::<Vec<Document>>())
    }

    fn generate_project_capabilities(&self) -> Result<Vec<Document>> {
        let project_kinds = vec![ProjectCapability {
            type_name: "led".to_string(),
            name: "LED - Light".to_string(),
            kinds: ["indicator".to_string()].to_vec(),
        }, ProjectCapability {
            type_name: "wifi".to_string(),
            name: "WiFi".to_string(),
            kinds: ["indicator".to_string(), "sensor".to_string()].to_vec(),
        }, ProjectCapability {
            type_name: "speaker".to_string(),
            name: "Speaker".to_string(),
            kinds: ["indicator".to_string()].to_vec(),
        }, ProjectCapability {
            type_name: "mic".to_string(),
            name: "Microphone".to_string(),
            kinds: ["sensor".to_string()].to_vec(),
        }];

        let bson_entities: Vec<Bson> = project_kinds.iter()
            .map(|entity| bson::to_bson(&entity).map_err(|e| e.into()))
            .collect::<Result<Vec<Bson>>>()?;

        Ok(bson_entities.into_iter().filter_map(|entity| {
            if let Bson::Document(document) = entity { Some(document) } else { None }
        }).collect::<Vec<Document>>())
    }
}