#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashSet, fs::File, io::Read, thread, time::Duration};

use activity::*;
use chrono::{NaiveDateTime, Utc};

use activity_manager::{
    data_handler, get_name_from_loc,
    user_settings::{ApplicationData, USER_SETTINGS},
    OkRequest, SEEN, SEEN_LOCAL,
};
use reqwest::{
    multipart::{Form, Part},
    Client,
};
use serde_json::json;
use tauri::{Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent};

async fn upon_watch_end(new: (String, NaiveDateTime)) {
    let now = Utc::now().naive_utc();

    let future = new.1;

    let diff = Duration::from_millis((now - future).num_milliseconds() as u64);

    let server = USER_SETTINGS.read().unwrap().server.clone();
    OkRequest! {
        Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key).json(&json!({
          "active": false,
          "activity": get_name_from_loc(&new.0),
          "time": diff.as_secs()/60,
        })).send().await.unwrap()
    }

    SEEN_LOCAL.write().unwrap().remove(&new.0);
}

async fn upon_watch_start(new: (String, NaiveDateTime)) {
    SEEN_LOCAL.write().unwrap().insert(new.0.clone(), new.1);

    let now = Utc::now().naive_utc();

    let future = new.1;

    let diff = Duration::from_millis((now - future).num_milliseconds() as u64);

    let server = USER_SETTINGS.read().unwrap().server.clone();
    OkRequest! {
      Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key).json(&json!({
        "active": true,
        "activity": get_name_from_loc(&new.0),
        "time": diff.as_secs()/60,
      })).send().await.unwrap()
    }
}

type SeenData = (String, NaiveDateTime);
#[tokio::main]
async fn main() {
    let tray_menu = SystemTrayMenu::new();
    {
        let set = USER_SETTINGS.read().unwrap();
        // let mut hm = HeaderMap::new();
        // hm.insert("ApiKey", api_key.parse().unwrap());
        let client = reqwest::Client::new();
        for app in set.applications.clone() {
            let mut ico = File::open(app.icon_location.as_str()).unwrap();
            let mut icob = vec![];
            ico.read_to_end(&mut icob).unwrap();
            // println!("{:?}",icob);
            let f = Form::new()
                .text("name", app.name.clone())
                .part("file", Part::bytes(icob).file_name("pog.png"));
            OkRequest! {
              if client.post(format!("{}/newApp",set.server.address)).multipart(f).header("ApiKey",set.server.api_key.clone()).send().await.is_err() {
                USER_SETTINGS.write().unwrap().server.address = "".to_string();
              }
            }
        }

        set.save().unwrap();
    }

    let context = tauri::generate_context!();
    let app = tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|a, e| match e {
            SystemTrayEvent::MenuItemClick { .. } => {}
            SystemTrayEvent::LeftClick { .. } => {
                let w = a.get_window("main").unwrap();
                if !w.is_visible().unwrap_or(false) {
                    w.show().unwrap();
                    w.set_focus().unwrap();
                } else {
                    w.hide().unwrap();
                }
            }
            _ => {}
        })
        .menu(if cfg!(target_os = "macos") {
            tauri::Menu::os_default(&context.package_info().name)
        } else {
            tauri::Menu::default()
        })
        .invoke_handler(tauri::generate_handler![
            data_handler::set_data,
            data_handler::get_data,
            data_handler::dir_exist,
            data_handler::set_api_key,
            data_handler::set_server_addr,
            new_application
        ])
        .setup(|a| {
            let w = a.get_window("main").unwrap();
            tokio::spawn(async move {
                loop {
                    let old: HashSet<SeenData> =
                        HashSet::from_iter(SEEN.read().unwrap().iter().cloned());
                    let mut watch = vec![];
                    {
                        let us = USER_SETTINGS.clone();
                        let set = us.read().unwrap();
                        println!("{:?}", set);
                        watch = set
                            .applications
                            .iter()
                            .map(|x| x.location.clone())
                            .collect::<Vec<_>>();
                    }
                    is_any_running(&watch, &mut SEEN.write().unwrap());
                    let new: HashSet<SeenData> =
                        HashSet::from_iter(SEEN.read().unwrap().iter().cloned());
                    let removed: Vec<&SeenData> = old.difference(&new).collect::<Vec<&SeenData>>();
                    let added: Vec<&SeenData> = new.difference(&old).collect::<Vec<&SeenData>>();

                    for a in added {
                        println!("Added: {:?}", a);
                        upon_watch_start(a.clone()).await
                    }
                    for r in removed {
                        println!("Removed: {:?}", r);
                        upon_watch_end(r.clone()).await
                    }

                    let sl = SEEN_LOCAL.read().unwrap();
                    println!("SL{:?}", sl);

                    w.emit("backend://activity-update", sl.clone()).unwrap();

                    thread::sleep(Duration::from_secs(3));
                }
            });

            println!("done startup!");

            Ok(())
        })
        .build(context)
        .expect("error while running tauri application");

    app.run(|app_handle,e| match e {
    RunEvent::WindowEvent {
      
      event: WindowEvent::CloseRequested { api, .. },
      ..
    } => {
      let app_handle = app_handle.clone();
      api.prevent_close();
      let us = USER_SETTINGS.clone();
      tokio::spawn( async move {

        let mut apps = vec![];
        let server;
        {
          apps = us.read().unwrap().applications.clone();
          server = us.read().unwrap().server.clone();
        }

        for app in apps {
          OkRequest!{
            Client::new().patch(format!("{}/app",server.address)).header("ApiKey", server.api_key.clone()).json(&json!({
              "active": false,
              "activity": get_name_from_loc(app.location.clone().as_str()),
              "time": 0,
            })).send().await.unwrap()
          }
        }

        app_handle.get_window("main").unwrap().close().unwrap();
      });
    }
    
    _=>{
      
    }
  })
}

#[tauri::command]
async fn new_application(location: String, name: String, icon: String) -> Result<(), String> {
    let server;
    {
        let set = USER_SETTINGS.clone();
        let mut set = set.write().unwrap();
        set.applications.push(ApplicationData {
            location: location.to_string(),
            name: name.to_string(),
            icon_location: icon.to_string(),
        });

        server = set.server.clone();

        set.save().unwrap();
    }

    let mut ico = File::open(icon.as_str()).unwrap();
    let mut icob = vec![];
    ico.read_to_end(&mut icob).unwrap();
    // println!("{:?}",icob);
    let f = Form::new()
        .text("name", name.clone())
        .part("file", Part::bytes(icob).file_name("pog.png"));
    OkRequest! {
      Client::new().post(format!("{}/newApp",server.address)).multipart(f).header("ApiKey",server.api_key).send().await.unwrap()
    }
    Ok(())
}
