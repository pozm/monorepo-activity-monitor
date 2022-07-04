use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use chrono::NaiveDateTime;
use user_settings::USER_SETTINGS;

pub mod data_handler;
pub mod user_settings;
use lazy_static::lazy_static;

#[macro_export]
macro_rules! OkRequest {
    ($e:expr) => {
        if !USER_SETTINGS.read().unwrap().server.address.is_empty() {
            $e;
        }
    };
}

pub fn get_name_from_loc(loc: &str) -> String {
    let set = USER_SETTINGS.read().unwrap();
    set.applications
        .iter()
        .find(|app| app.location == loc)
        .unwrap()
        .name
        .clone()
}

lazy_static! {
    pub static ref SEEN: Arc<RwLock<Vec<(String, NaiveDateTime)>>> = Arc::new(RwLock::new(vec![]));
    pub static ref SEEN_LOCAL: Arc<RwLock<HashMap<String, NaiveDateTime>>> =
        Arc::new(RwLock::new(HashMap::new()));
}
