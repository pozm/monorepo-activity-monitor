use std::{collections::HashMap, path::PathBuf, sync::{RwLock, RwLockWriteGuard}, ffi::OsString, str::FromStr};

use reqwest::Client;
use serde_json::json;
use crate::HOSTNAME_;

use crate::{
    get_name_from_loc,
    user_settings::{UserSettings, USER_SETTINGS},
    OkRequest, SEEN, SEEN_LOCAL,
};

#[tauri::command]
pub async fn get_data() -> Result<UserSettings, String> {
    let data = USER_SETTINGS.read().or(Err("unable to open settings"))?;
    Ok((*data).clone())
}
#[tauri::command]
pub async fn set_data(new: UserSettings) -> Result<(), String> {
    new.save()?;
    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    *dat = new;
    Ok(())
}

#[tauri::command]
pub async fn dir_exist(dir: PathBuf) -> bool {
    dir.exists()
}
#[tauri::command]
pub async fn set_api_key(mut st: String) -> Result<(), String> {
    let server;
    {
        let us = USER_SETTINGS.clone();
        server = us.read().unwrap().server.clone();
    }

    for app in &server.applications {
        OkRequest! {
          if Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key.clone()).json(&json!({
              "active": false,
              "activity": get_name_from_loc(app.location.clone().as_str()),
              "time": 0,
              "dName":HOSTNAME_.lock().unwrap().clone()
            })).send().await.is_err() {
                st = "".to_string();
            }
        }
    }

    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    (*dat).server.api_key = st.clone();
    (*dat).save();
    *SEEN.write().unwrap() = vec![];
    *SEEN_LOCAL.write().unwrap() = HashMap::new();
    Ok(())
}
#[tauri::command]
pub async fn set_server_addr(mut st: String) -> Result<(), String> {
    let server;
    {
        let us = USER_SETTINGS.clone();
        server = us.read().unwrap().server.clone();
    }
    println!("{:?}", server);
    for app in &server.applications {
        OkRequest! {
          if Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key.clone()).json(&json!({
            "active": false,
            "activity": get_name_from_loc(app.location.clone().as_str()),
            "time": 0,
            "dName":HOSTNAME_.lock().unwrap().clone()
          })).send().await.is_err() {
            //   st = "".to_string();
            println!("current is bricked.")
          }
        }
    }
    let key = server.api_key.clone();
    let userdata;
    {

        let mut dat : RwLockWriteGuard<UserSettings> = USER_SETTINGS.write().or(Err("unable to open settings"))?;
        println!("{:?}", st);
        (*dat).server.address = st.clone();
        (*dat).save();
        userdata = (*dat).clone();
    }
    userdata.announce_hello().await;
    // {

    OkRequest! {
        if Client::new().post(format!("{}/device/new?name={}",st.clone(),HOSTNAME_.lock().unwrap().clone())).header("ApiKey",key).send().await.is_err() {
            //   st = "".to_string();
            println!("current is bricked.")
        }
    }

    // }

    *SEEN.write().unwrap() = vec![];
    *SEEN_LOCAL.write().unwrap() = HashMap::new();

    Ok(())
}
