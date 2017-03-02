use core;
use bson;
use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use errors::*;

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
    pub fn get_control_groups(&self) -> Result<Vec<core::control_group::ControlGroup>> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let mut result: Vec<core::control_group::ControlGroup> = vec![];
        let cursor = db.collection("groups").find(None, None)?;
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

        if groups_collection.find_one(None, None)?.is_some() {
            info!("Database is ready.");
            return Ok(());
        }

        info!("Database is empty, filling it with the data...");

        let control_groups = vec![core::control_group::ControlGroup {
            type_name: "group#1".to_string(),
            name: "Group #1".to_string(),
            description: "Group #1 Description".to_string(),
            items: vec![core::control_metadata::ControlMetadata {
                type_name: "type#11".to_string(),
                name: "Item #11".to_string(),
                description: "Item #11 Description".to_string(),
            }, core::control_metadata::ControlMetadata {
                type_name: "type#12".to_string(),
                name: "Item #12".to_string(),
                description: "Item #12 Description".to_string(),
            }],
        }, core::control_group::ControlGroup {
            type_name: "group#2".to_string(),
            name: "Group #2".to_string(),
            description: "Group #2 Description".to_string(),
            items: vec![core::control_metadata::ControlMetadata {
                type_name: "type#21".to_string(),
                name: "Item #21".to_string(),
                description: "Item #21 Description".to_string(),
            }, core::control_metadata::ControlMetadata {
                type_name: "type#22".to_string(),
                name: "Item #22".to_string(),
                description: "Item #22 Description".to_string(),
            }],
        }];

        let bson_entities: Vec<Bson> = control_groups.iter()
            .map(|group| bson::to_bson(&group).map_err(|e| e.into()))
            .collect::<Result<Vec<Bson>>>()?;

        let documents = bson_entities.into_iter().filter_map(|entity| {
            if let Bson::Document(document) = entity { Some(document) } else { None }
        }).collect::<Vec<Document>>();

        groups_collection.insert_many(documents, None)?;

        Ok(())
    }
}