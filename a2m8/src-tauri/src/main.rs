#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use directories::ProjectDirs;
use tauri::{
    async_runtime::Mutex, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu, Wry,
};
use tokio::fs;

macro_rules! import_modules {
    ($($x:ident),*) => {
        $(
            mod $x;
        )*
    };
}

import_modules! {
    a2m8_config,
    commands,
    error,
    prelude,
    script
}

use crate::{commands::*, prelude::*};

fn create_tray(scripts: &Vec<A2M8Script>) -> Result<SystemTray> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let mut starter_menu = SystemTrayMenu::new();
    let mut stop_menu = SystemTrayMenu::new();

    let mut scripts = scripts.clone();
    scripts.sort_by_key(|s| s.favorite);
    for script in scripts {
        if !script.running() {
            let start = CustomMenuItem::new(script.id.to_string(), &format!("Start {}", script.name));
            starter_menu = starter_menu.add_item(start);
        } else {
            let stop = CustomMenuItem::new(script.id.to_string(), &format!("Stop {}", script.name));
            stop_menu = stop_menu.add_item(stop);
        }
    }
    if starter_menu.items.is_empty() {
        starter_menu = starter_menu.add_native_item(SystemTrayMenuItem::Separator);
        starter_menu =
            starter_menu.add_item(CustomMenuItem::new("cool_id", "No scripts to start".to_string()).disabled());
    }
    if stop_menu.items.is_empty() {
        stop_menu = stop_menu.add_native_item(SystemTrayMenuItem::Separator);
        stop_menu = stop_menu.add_item(CustomMenuItem::new("cool_id", "No scripts to stop".to_string()).disabled());
    }

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(open)
        .add_submenu(SystemTraySubmenu::new("start script", starter_menu))
        .add_submenu(SystemTraySubmenu::new("stop script", stop_menu));

    let system_tray = SystemTray::new().with_menu(tray_menu);
    Ok(system_tray)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let dirs = ProjectDirs::from("dev", "tricked", "A2M8").unwrap();
    let path = dirs.data_dir().to_path_buf();
    fs::create_dir_all(&path).await?;
    let mut config = A2M8Config {
        scripts: Vec::new(),
        script_handles: Vec::new(),
        data_dir: dirs.data_dir().to_path_buf(),
    };
    config.load_scripts().await?;

    for script in &mut config.scripts {
        if script.startup {
            let handle = script.start().await?;
            config.script_handles.push(handle);
        }
    }

    let app = tauri::Builder::default()
        .system_tray(create_tray(&config.scripts)?)
        .manage(Mutex::new(config))
        .on_system_tray_event(handle_tray_event)
        .invoke_handler(tauri::generate_handler![
            create_script,
            update_script,
            delete_script,
            start_script,
            stop_script,
            get_scripts,
            get_script
        ]);
    app.run(tauri::generate_context!())?;
    Ok(())
}

fn handle_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _, size: _, ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _, size: _, ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _, size: _, ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "open" => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
