use bincode::de::read::Reader;
use bincode::enc::write::Writer;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use tauri::Runtime;
use tauri::api::path::{data_dir, local_data_dir};
use tokio::fs as fs_tokio;
use std::fs::{File, create_dir_all, self, remove_file};
use std::sync::Arc;
use std::{sync::RwLock, path::PathBuf, fs::OpenOptions};
use std::io::prelude::*;

lazy_static!{
	pub static ref USER_SETTINGS: Arc<RwLock<UserSettings>> = Arc::new(RwLock::new(UserSettings::read().unwrap()));
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct UserSettings {
	pub applications: Vec<ApplicationData>,
	pub server : ServerSettings
}
#[derive(Debug, Serialize, Deserialize,Clone)]

pub struct ApplicationData {
	pub location:String,
	pub name:String,
	pub icon_location:String,
}
#[derive(Debug, Serialize, Deserialize,Clone)]

pub struct ServerSettings {
	pub address:String,
	pub api_key:String
}

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


impl UserSettings{
	pub fn new() -> Self {
		let local_dir = local_data_dir().expect("unable to get document directory");
		fs::create_dir_all(&local_dir.join("activity-monitor")).expect("failed to create launcher directory");
		Self{
			applications: vec![],
			server:ServerSettings { address: "".to_string(), api_key: "".to_string() }
		}
	}
	pub fn get_save_dir() -> Result<PathBuf,String> {

		let data = data_dir().ok_or("Could not get data dir")?;
	
		Ok(data.join(SAVE_TO_PATH))
	}

	pub fn read() -> Result<Self,String> {
		let data_path = Self::get_save_dir()?;

		create_dir_all(data_path.parent().unwrap()).or(Err("Could not create launcher dir".to_string()))?;

		let data_file = OpenOptions::new().read(true).create(true).write(true).open(data_path).or(Err("Unable to make data file"))?;
		let config = bincode::config::standard();
		let mut return_data : Self = match bincode::serde::decode_from_reader(FileWrapper(data_file), config) {
			Ok(ret_data) => ret_data,
			Err(e) => {
				println!("unable to read data file: {} | returning defaults.",e);
				Self::new() //
			},
		};
		// println!("got data {:#?}",return_data);
		Ok(return_data)
	}

	pub fn save(&self) -> Result<(),String> {
		
		let data_path = Self::get_save_dir()?;

		// println!("save data {:#?}",&self);

		create_dir_all(data_path.parent().unwrap()).or(Err("Could not create launcher dir".to_string()))?;

		let data_file = OpenOptions::new()
			.write(true)
			.create(true)
			.truncate(true)
			.open(data_path).or(Err("Unable to make data file"))?;
		let config = bincode::config::standard();
		bincode::serde::encode_into_writer(&self, FileWrapper(data_file), config).or(Err("Unable to write data file"))?;

		Ok(())
	}
	pub fn reset() -> Result<(),String> {
		let data_path = Self::get_save_dir()?;
		remove_file(data_path).or(Err("Unable to remove data file"))?;
		Ok(())

	}
}