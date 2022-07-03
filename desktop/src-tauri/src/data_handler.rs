use std::{path::PathBuf, sync::Arc, collections::HashMap};

use reqwest::Client;
use serde_json::json;

use crate::{user_settings::{UserSettings, USER_SETTINGS}, OkRequest, get_name_from_loc, SEEN, SEEN_LOCAL};

#[tauri::command]
pub async fn get_data(
) -> Result<UserSettings, String> {
    let data = USER_SETTINGS
        .read()
        .or(Err("unable to open settings"))?;
    Ok((*data).clone())
}
#[tauri::command]
pub async fn set_data(
    new: UserSettings,
) -> Result<(), String> {
    new.save()?;
    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    *dat = new;
    Ok(())
}

#[tauri::command]
pub async fn dir_exist(dir:PathBuf) -> bool {
  dir.exists()
}
#[tauri::command]
pub async fn set_api_key(mut st:String) -> Result<(),String> {

    let mut apps = vec![];
    let server;
    {
        let us = USER_SETTINGS.clone();
        apps = us.read().unwrap().applications.clone();
        server = us.read().unwrap().server.clone();
    }

    for app in apps {
      OkRequest!{
        if Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key.clone()).json(&json!({
            "active": false,
            "activity": get_name_from_loc(app.location.clone().as_str()),
            "time": 0,
          })).send().await.is_err() {
              st = "".to_string();
          }
      }
    }

    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    (*dat).server.api_key = st.clone();
    *SEEN.write().unwrap() = vec![];
    *SEEN_LOCAL.write().unwrap() = HashMap::new();
    Ok(())
}
#[tauri::command]
pub async fn set_server_addr(mut st:String) -> Result<(),String> {

    let mut apps = vec![];
    let server;
    {
        let us = USER_SETTINGS.clone();
        apps = us.read().unwrap().applications.clone();
        server = us.read().unwrap().server.clone();
    }
    println!("{:?}",server);
    for app in apps {
      OkRequest!{
        if Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key.clone()).json(&json!({
          "active": false,
          "activity": get_name_from_loc(app.location.clone().as_str()),
          "time": 0,
        })).send().await.is_err() {
            st = "".to_string();
        }
      }
    }

    let mut dat = USER_SETTINGS.write().or(Err("unable to open settings"))?;
    (*dat).server.address = st.clone();

    *SEEN.write().unwrap() = vec![];
    *SEEN_LOCAL.write().unwrap() = HashMap::new();

    Ok(())
}