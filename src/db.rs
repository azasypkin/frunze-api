use bson;
use bson::{Bson, Document};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use errors::*;
use serde;

use editor::control_group::ControlGroup;
use editor::control_metadata::ControlMetadata;
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

        // TODO: Move this to a docker initialization script.
        self.fill_db_if_empty()?;

        Ok(())
    }

    /// Queries project instance from the database using passed `project_id`.
    pub fn get_project(&self, project_id: &str) -> Result<Option<Project>> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let result = db.collection("projects").find_one(Some(doc! { "id" => project_id }), None)?;
        let result = if let Some(project) = result {
            Some(bson::from_bson(bson::Bson::Document(project))?)
        } else {
            None
        };

        Ok(result)
    }

    /// Saves project to the database.
    pub fn save_project(&self, project: &Project) -> Result<()>{
        let db = self.client.as_ref().unwrap().db(&self.name);

        let collection = db.collection("projects");
        if let Bson::Document(document) = bson::to_bson(&project)? {
            collection.insert_one(document, None)?;
        }

        Ok(())
    }

    /// Queries control groups from the database.
    pub fn get_control_groups(&self) -> Result<Vec<ControlGroup>> {
        self.get_collection("groups")
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
        where T: serde::de::DeserializeOwned {
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

    fn fill_db_if_empty(&self) -> Result<()> {
        self.set_collection("groups", || {
            info!("Filling DB with the control groups...");
            self.generate_control_groups()
        })?;

        self.set_collection("project_capabilities", || {
            info!("Filling DB with the project capabilities...");
            self.generate_project_capabilities()
        })?;

        self.set_collection("project_capability_groups", || {
            info!("Filling DB with the project capability groups...");
            self.generate_project_capability_groups()
        })?;

        self.set_collection("project_platforms", || {
            info!("Filling DB with the project platforms...");
            self.generate_project_platforms()
        })?;

        info!("Database is ready.");

        Ok(())
    }

    // TODO: Below are temporary methods that will be replaced by the pre-loaded DB files.
    fn generate_control_groups(&self) -> Vec<ControlGroup> {
        let control_groups = vec![ControlGroup {
            type_name: "hardware-connection".to_string(),
            name: "Hardware - Connection".to_string(),
            description: "All applicable connection related hardware components.".to_string(),
            items: vec![ControlMetadata {
                type_name: "hardware-connection-wifi".to_string(),
                name: "WiFi Connection".to_string(),
                description: "Enables WiFi connectivity.".to_string(),
            }, ControlMetadata {
                type_name: "hardware-connection-uart".to_string(),
                name: "UART Connection".to_string(),
                description: "Enables UART connectivity.".to_string(),
            }],
        }, ControlGroup {
            type_name: "hardware-audio".to_string(),
            name: "Hardware - Audio".to_string(),
            description: "All applicable audio related hardware components.".to_string(),
            items: vec![ControlMetadata {
                type_name: "hardware-audio-speaker".to_string(),
                name: "Speaker".to_string(),
                description: "Allows your device to play sounds and music.".to_string(),
            }, ControlMetadata {
                type_name: "hardware-audio-microphone".to_string(),
                name: "Microphone".to_string(),
                description: "Allows your device to listen and record audio.".to_string(),
            }],
        }, ControlGroup {
            type_name: "hardware-switches".to_string(),
            name: "Hardware - Switches".to_string(),
            description: "Various hardware switches.".to_string(),
            items: vec![ControlMetadata {
                type_name: "hardware-momentary-switch".to_string(),
                name: "Momentary Switch".to_string(),
                description: "Allows your device to have a mechanical momentary switch."
                    .to_string(),
            }, ControlMetadata {
                type_name: "hardware-maintained-switch".to_string(),
                name: "Maintained Switch".to_string(),
                description: "Allows your device to have a mechanical switch with a maintained \
                    state.".to_string(),
            }],
        }, ControlGroup {
            type_name: "services-storage".to_string(),
            name: "Services - Storage".to_string(),
            description: "Various storage related services.".to_string(),
            items: vec![ControlMetadata {
                type_name: "services-cloud-storage".to_string(),
                name: "Cloud Storage".to_string(),
                description: "Allows your device to have access to a cloud storage.".to_string(),
            }, ControlMetadata {
                type_name: "services-storage-adapter".to_string(),
                name: "Storage Adapter".to_string(),
                description: "Allows device to do various transformation on the data retrieved \
                    from particular storage.".to_string(),
            }],
        }];

        control_groups
    }

    fn generate_project_capabilities(&self) -> Vec<ProjectCapability> {
        let project_capabilities = vec![ProjectCapability {
            type_name: "wifi".to_string(),
            name: "WiFi Connection".to_string(),
            description: "Enables WiFi connectivity.".to_string()
        }, ProjectCapability {
            type_name: "ethernet".to_string(),
            name: "Ethernet Connection".to_string(),
            description: "Enables Ethernet connectivity.".to_string()
        }, ProjectCapability {
            type_name: "bluetooth".to_string(),
            name: "Bluetooth Connection".to_string(),
            description: "Enables Bluetooth connectivity.".to_string()
        }, ProjectCapability {
            type_name: "cellular".to_string(),
            name: "Cellular connection".to_string(),
            description: "Enables Cellular connectivity.".to_string()
        }, ProjectCapability {
            type_name: "nfc".to_string(),
            name: "NFC connection".to_string(),
            description: "Enables Near field communication connectivity.".to_string()
        }, ProjectCapability {
            type_name: "lora".to_string(),
            name: "LoRa WAN connection".to_string(),
            description: "Enables LoRa WAN connectivity.".to_string()
        }, ProjectCapability {
            type_name: "uart".to_string(),
            name: "UART connection".to_string(),
            description: "Enables UART connectivity.".to_string()
        }, ProjectCapability {
            type_name: "led".to_string(),
            name: "LED Indicator".to_string(),
            description: "Allows your device to light up LEDs whenever needed.".to_string()
        }, ProjectCapability {
            type_name: "led-display".to_string(),
            name: "LED Display".to_string(),
            description: "Allows your device to use basic LED display.".to_string()
        }, ProjectCapability {
            type_name: "lcd-display".to_string(),
            name: "Display".to_string(),
            description: "Allows your device to use LCD.".to_string()
        }, ProjectCapability {
            type_name: "speaker".to_string(),
            name: "Speaker".to_string(),
            description: "Allows your device to play sounds and music.".to_string()
        }, ProjectCapability {
            type_name: "beeper".to_string(),
            name: "Beeper".to_string(),
            description: "Allows your device to produce very basic sounds or beeps.".to_string()
        }, ProjectCapability {
            type_name: "mic".to_string(),
            name: "Microphone".to_string(),
            description: "Allows your device to listen and record audio.".to_string()
        }, ProjectCapability {
            type_name: "light".to_string(),
            name: "Light Sensor".to_string(),
            description: "Allows your device to react on changes in light conditions.".to_string()
        }, ProjectCapability {
            type_name: "humidity".to_string(),
            name: "Humidity Sensor".to_string(),
            description: "Allows your device to measure humidity level.".to_string()
        }, ProjectCapability {
            type_name: "motion".to_string(),
            name: "Motion Sensor".to_string(),
            description: "Allows your device to detect motion.".to_string()
        }, ProjectCapability {
            type_name: "temperature".to_string(),
            name: "Temperature Sensor".to_string(),
            description: "Allows your device to measure temperature.".to_string()
        }, ProjectCapability {
            type_name: "magnetic".to_string(),
            name: "Magnetic Field Sensor".to_string(),
            description: "Allows your device to detect changes in magnetic field.".to_string()
        }, ProjectCapability {
            type_name: "air-pollution".to_string(),
            name: "Air Pollution Sensor".to_string(),
            description: "Allows your device to measure air pollution level.".to_string()
        }, ProjectCapability {
            type_name: "dc-motor".to_string(),
            name: "DC Motor".to_string(),
            description: "Allows your device to spin a DC motor.".to_string()
        }, ProjectCapability {
            type_name: "servo-motor".to_string(),
            name: "Servo Motor".to_string(),
            description: "Allows your device to control a servo motor.".to_string()
        }, ProjectCapability {
            type_name: "vibration-motor".to_string(),
            name: "Vibration motor".to_string(),
            description: "Allows your device to vibrate.".to_string()
        }, ProjectCapability {
            type_name: "momentary-switch".to_string(),
            name: "Momentary Switch".to_string(),
            description: "Allows your device to have a mechanical momentary switch.".to_string()
        }, ProjectCapability {
            type_name: "maintained-switch".to_string(),
            name: "Maintained Switch".to_string(),
            description: "Allows your device to have a mechanical switch with a maintained state."
                .to_string()
        }, ProjectCapability {
            type_name: "rotary-switch".to_string(),
            name: "Rotary Switch".to_string(),
            description: "Allows your device to have a rotary switch.".to_string()
        }];

        project_capabilities
    }

    fn generate_project_capability_groups(&self) -> Vec<ProjectCapabilityGroup> {
        let project_capability_groups = vec![ProjectCapabilityGroup {
            type_name: "connectivity".to_string(),
            name: "Connectivity".to_string(),
            description: "Everything that allows devices to be connected.".to_string(),
            capabilities: vec!["wifi".to_string(), "ethernet".to_string(), "bluetooth".to_string(),
                               "cellular".to_string(), "nfc".to_string(), "lora".to_string()]
        }, ProjectCapabilityGroup {
            type_name: "output".to_string(),
            name: "Indicators and Output".to_string(),
            description: "Items in this group allows your device to talk to the world or indicate \
                something (various audio/visual indicators).".to_string(),
            capabilities: vec!["led".to_string(), "led-display".to_string(),
                               "lcd-display".to_string(), "speaker".to_string(),
                               "beeper".to_string()]
        }, ProjectCapabilityGroup {
            type_name: "input".to_string(),
            name: "Sensors and Input".to_string(),
            description: "Items in this group allows your device to get input from the world (various \
                sensors, microphones etc.).".to_string(),
            capabilities: vec!["mic".to_string(), "light".to_string(), "humidity".to_string(),
                               "motion".to_string(), "temperature".to_string(),
                               "magnetic".to_string(), "air-pollution".to_string()]
        }, ProjectCapabilityGroup {
            type_name: "mechanical".to_string(),
            name: "Mechanical".to_string(),
            description: "Items in this group allows your device to perform mechanical \
                actions.".to_string(),
            capabilities: vec!["dc-motor".to_string(), "servo-motor".to_string(),
                               "vibration-motor".to_string()]
        }, ProjectCapabilityGroup {
            type_name: "switches".to_string(),
            name: "Switches".to_string(),
            description: "Items in this group allows your device to have various switches."
                .to_string(),
            capabilities: vec!["momentary-switch".to_string(), "maintained-switch".to_string(),
                               "rotary-switch".to_string()]
        }];

        project_capability_groups
    }

    fn generate_project_platforms(&self) -> Vec<ProjectPlatform> {
        let project_platforms = vec![ProjectPlatform {
            type_name: "attiny".to_string(),
            name: "Atmel ATtiny85".to_string(),
            description: "High Performance, Low Power AVR® 8-Bit Microcontroller.".to_string(),
            capabilities: vec!["uart".to_string(), "mic".to_string(), "beeper".to_string(),
                               "light".to_string(), "humidity".to_string(), "motion".to_string(),
                               "temperature".to_string(), "magnetic".to_string(),
                               "momentary-switch".to_string(), "maintained-switch".to_string(),
                               "rotary-switch".to_string()]
        }, ProjectPlatform {
            type_name: "raspberry-pi-zero-w".to_string(),
            name: "The Raspberry Pi Zero W".to_string(),
            description: "The Raspberry Pi Zero W Microcomputer.".to_string(),
            capabilities: vec!["uart".to_string(), "wifi".to_string(), "bluetooth".to_string(),
                               "led".to_string(), "speaker".to_string(), "beeper".to_string(),
                               "mic".to_string(), "light".to_string(), "humidity".to_string(),
                               "motion".to_string(), "temperature".to_string(),
                               "magnetic".to_string(), "momentary-switch".to_string(),
                               "maintained-switch".to_string(), "rotary-switch".to_string()]
        }, ProjectPlatform {
            type_name: "raspberry-pi-3".to_string(),
            name: "The Raspberry Pi 3".to_string(),
            description: "The Raspberry Pi 3 Microcomputer.".to_string(),
            capabilities: vec!["uart".to_string(), "wifi".to_string(), "bluetooth".to_string(),
                               "led".to_string(), "speaker".to_string(), "beeper".to_string(),
                               "mic".to_string(), "light".to_string(), "humidity".to_string(),
                               "motion".to_string(), "temperature".to_string(),
                               "magnetic".to_string(), "momentary-switch".to_string(),
                               "maintained-switch".to_string(), "rotary-switch".to_string()]
        }, ProjectPlatform {
            type_name: "psf-a85".to_string(),
            name: "PSF-A85 WiFi Module".to_string(),
            description: "PSF-A85 Wireless Module from iTEAD.".to_string(),
            capabilities: vec!["uart".to_string(), "wifi".to_string(), "bluetooth".to_string(),
                               "led".to_string(), "speaker".to_string(), "beeper".to_string(),
                               "mic".to_string(), "light".to_string(), "humidity".to_string(),
                               "motion".to_string(), "temperature".to_string(),
                               "magnetic".to_string(), "momentary-switch".to_string(),
                               "maintained-switch".to_string(), "rotary-switch".to_string()]
        }, ProjectPlatform {
            type_name: "tessel-2".to_string(),
            name: "Tessel 2".to_string(),
            description: "The Tessel 2 development board.".to_string(),
            capabilities: vec!["uart".to_string(), "wifi".to_string(), "bluetooth".to_string(),
                               "led".to_string(), "speaker".to_string(), "beeper".to_string(),
                               "mic".to_string(), "light".to_string(), "humidity".to_string(),
                               "motion".to_string(), "temperature".to_string(),
                               "magnetic".to_string(), "momentary-switch".to_string(),
                               "maintained-switch".to_string(), "rotary-switch".to_string()]
        }];

        project_platforms
    }

    fn set_collection<T, F>(&self, collection_name: &str, items_retriever: F) -> Result<()>
        where T: serde::Serialize, F: FnOnce() -> Vec<T> {
        let db = self.client.as_ref().unwrap().db(&self.name);

        let collection = db.collection(collection_name);
        if collection.find_one(None, None)?.is_none() {
            info!("Filling DB with the project platforms...");

            let bson_entities: Vec<Bson> = items_retriever().iter()
                .map(|entity| bson::to_bson(&entity).map_err(|e| e.into()))
                .collect::<Result<Vec<Bson>>>()?;

            let bson_documents = bson_entities.into_iter().filter_map(|entity| {
                if let Bson::Document(document) = entity { Some(document) } else { None }
            }).collect::<Vec<Document>>();

            collection.insert_many(bson_documents, None)?;
        }

        Ok(())
    }
}
