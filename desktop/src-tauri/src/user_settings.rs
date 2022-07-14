use bincode::de::read::Reader;
use bincode::enc::write::Writer;

use lazy_static::lazy_static;
use reqwest::Client;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use serde_json::json;
use tauri::api::path::{data_dir, local_data_dir};

use std::fs::{self, create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::sync::Arc;
use std::{fs::OpenOptions, path::PathBuf, sync::RwLock};

use crate::{OkRequest, get_name_from_loc, HOSTNAME_};

lazy_static! {
    pub static ref USER_SETTINGS: Arc<RwLock<UserSettings>> =
        Arc::new(RwLock::new(UserSettings::read().unwrap()));
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSettings {
    pub server: ServerSettings,
}
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct ApplicationData {
    pub location: String,
    pub name: String,
    pub icon_location: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct ServerSettings {
    pub address: String,
    pub api_key: String,
    pub applications: Vec<ApplicationData>,
}
#[cfg(debug_assertions)]
const SAVE_TO_PATH: &str = "activity-monitor/data-dev.bin";
#[cfg(not(debug_assertions))]
const SAVE_TO_PATH: &str = "activity-monitor/data.bin";

struct FileWrapper(File);

impl Writer for FileWrapper {
    fn write(&mut self, bytes: &[u8]) -> Result<(), bincode::error::EncodeError> {
        self.0.write(bytes).unwrap();
        Ok(())
    }
}
impl Reader for FileWrapper {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), bincode::error::DecodeError> {
        self.0.read(bytes).unwrap();
        Ok(())
    }
}

impl UserSettings {

    // server fns

    pub async fn announce_hello(&self) {
        if self.server.address.is_empty() {return}
        let client = reqwest::Client::new();
        for app in &self.server.applications.clone() {
            let mut ico = File::open(app.icon_location.as_str()).unwrap();
            let mut icob = vec![];
            ico.read_to_end(&mut icob).unwrap();
            // println!("{:?}",icob);
            let f = Form::new()
                .text("name", app.name.clone())
                .part("file", Part::bytes(icob).file_name("pog.png"));
              if client.post(format!("{}/app/new",self.server.address)).multipart(f).header("ApiKey",self.server.api_key.clone()).send().await.is_err() {
                // self.server.address = "".to_string();
              }
        }
    }
    pub async fn announce_goodbye(&self) {
        if self.server.address.is_empty() {return}
        let client = Client::new();
        for app in &self.server.applications {
              if client.patch(format!("{}/app",self.server.address)).header("ApiKey", self.server.api_key.clone()).json(&json!({
                "active": false,
                "activity": get_name_from_loc(app.location.clone().as_str()),
                "time": 0,
                "dName": HOSTNAME_.lock().unwrap().clone()
              })).send().await.is_err() {
                //   self.server.address = "".to_string();
              }
        }
    }
    pub async fn app_new(&self,appn:&str,loc:&str) {
        let mut ico = File::open(loc).unwrap();
        let mut icob = vec![];
        ico.read_to_end(&mut icob).unwrap();
        // println!("{:?}",icob);
        let f = Form::new()
            .text("name", appn.to_string())
            .part("file", Part::bytes(icob).file_name("pog.png"));
        if Client::new().post(format!("{}/app/new",self.server.address)).multipart(f).header("ApiKey",self.server.api_key.clone()).send().await.is_err() {
            // self.server.address = "".to_string();
        }
    }
    pub async fn app_delete(&self,appn:&str) {
        if self.server.address.is_empty() {return}
        if Client::new().delete(format!("{}/app/{}",self.server.address,appn)).send().await.is_err() {
            // self.server.address = "".to_string();
        };
    }


    // general data manip


    pub fn new() -> Self {
        let local_dir = local_data_dir().expect("unable to get document directory");
        fs::create_dir_all(&local_dir.join("activity-monitor"))
            .expect("failed to create launcher directory");
        Self {
            server: ServerSettings {
                address: "".to_string(),
                api_key: "".to_string(),
                applications: vec![],
            },
        }
    }
    pub fn get_save_dir() -> Result<PathBuf, String> {
        let data = data_dir().ok_or("Could not get data dir")?;

        Ok(data.join(SAVE_TO_PATH))
    }

    pub fn read() -> Result<Self, String> {
        let data_path = Self::get_save_dir()?;

        create_dir_all(data_path.parent().unwrap())
            .or(Err("Could not create launcher dir".to_string()))?;

        let data_file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .open(data_path)
            .or(Err("Unable to make data file"))?;
        let config = bincode::config::standard();
        let return_data: Self =
            match bincode::serde::decode_from_reader(FileWrapper(data_file), config) {
                Ok(ret_data) => ret_data,
                Err(e) => {
                    println!("unable to read data file: {} | returning defaults.", e);
                    Self::new() //
                }
            };
        // println!("got data {:#?}",return_data);
        Ok(return_data)
    }

    pub fn save(&self) -> Result<(), String> {
        let data_path = Self::get_save_dir()?;

        // println!("save data {:#?}",&self);

        create_dir_all(data_path.parent().unwrap())
            .or(Err("Could not create launcher dir".to_string()))?;

        let data_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(data_path)
            .or(Err("Unable to make data file"))?;
        let config = bincode::config::standard();
        bincode::serde::encode_into_writer(&self, FileWrapper(data_file), config)
            .or(Err("Unable to write data file"))?;

        Ok(())
    }
    pub fn reset() -> Result<(), String> {
        let data_path = Self::get_save_dir()?;
        remove_file(data_path).or(Err("Unable to remove data file"))?;
        Ok(())
    }
}
