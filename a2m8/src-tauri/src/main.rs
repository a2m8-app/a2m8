#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::{path::Path, sync::Arc};

use crate::{commands::*, prelude::*};

use clap::Parser;
use directories::ProjectDirs;

use tauri::{
    async_runtime::Mutex, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem, SystemTraySubmenu, Wry,
};
use tokio::{fs, sync::mpsc};
use tracing::metadata::LevelFilter;
use tracing_subscriber::EnvFilter;
use utils::create_lua;

mod a2m8_config;
mod cli;
mod commands;
mod error;
mod http;
mod prelude;
mod script;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .without_time()
        .with_target(false)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let args = cli::Args::parse();

    let path = match args.data_dir {
        Some(path) => path,
        _ => {
            let dirs = ProjectDirs::from("dev", "tricked", "A2M8").unwrap();
            let path = dirs.data_dir().to_path_buf();
            path
        }
    };

    match args.subcommand {
        Some(cli::Command::Run { file }) => {
            let lua = utils::create_lua()?;
            lua.load(Path::new(&file))
                .set_name(file.file_name().unwrap().to_string_lossy())?
                .exec_async()
                .await?;
            Ok(())
        }

        _ => {
            fs::create_dir_all(&path).await?;

            let (tx, rx) = tokio::sync::mpsc::channel(10);

            let mut config = A2M8Config {
                scripts: Vec::new(),
                script_handles: Vec::new(),
                stop_sender: tx.clone(),
                data_dir: path,
            };
            config.load_scripts().await?;
            match args.subcommand {
                Some(cli::Command::List {}) => {
                    for script in &config.scripts {
                        println!("{} {}", script.id, script.name);
                    }
                    Ok(())
                }
                Some(cli::Command::Add { file }) => {
                    let script = A2M8Script::from_file(file)?;
                    config.scripts.push(script);
                    config.save_scripts().await?;
                    Ok(())
                }
                Some(cli::Command::Delete { id }) => {
                    config.scripts.retain(|script| script.id != id);
                    config.save_scripts().await?;
                    Ok(())
                }
                Some(cli::Command::Start { id }) => {
                    let script = config.scripts.iter_mut().find(|script| script.id == id);
                    if let Some(script) = script {
                        let lua = create_lua()?;
                        lua.load(&script.content).set_name(&script.name)?.exec_async().await?;
                    }
                    Ok(())
                }
                Some(cli::Command::Inspect { id }) => {
                    let script = config.scripts.iter_mut().find(|script| script.id == id);
                    if let Some(script) = script {
                        println!("name: {}", script.name);
                        println!("id: {}", script.id);
                        println!("favorite: {}", script.favorite);
                        println!("description: {}", script.description);
                        println!("content\n{}", script.content);
                    }
                    Ok(())
                }
                _ => start_app(config, (tx, rx)).await,
            }
        }
    }
}

async fn start_app(
    mut config: A2M8Config,
    (_tx, mut rx): (mpsc::Sender<ScriptEnd>, mpsc::Receiver<ScriptEnd>),
) -> Result<()> {
    for script in &mut config.scripts {
        if script.running() {
            script.status = A2M8Script::STATUS_STOPPED;
        }
    }
    //pretty nasty ngl
    for script in config.scripts.clone() {
        if script.startup {
            config.run_script(script).await?;
        }
    }

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let app = tauri::Builder::default()
        .system_tray(create_tray(&config.scripts)?)
        .manage(Arc::new(Mutex::new(config)))
        .on_system_tray_event(handle_tray_event)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            tokio::spawn(async move {
                while let Some(val) = rx.recv().await {
                    main_window.emit("script_end", val)?;
                }
                Ok::<_, error::Error>(())
            });
            let window = app.get_window("main").unwrap();
            let state = Arc::clone(&app.state::<A2>());
            tokio::spawn(async move { http::start_web(window, state).await });
            Ok(())
        })
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

fn create_tray(scripts: &[A2M8Script]) -> Result<SystemTray> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let mut starter_menu = SystemTrayMenu::new();
    let mut stop_menu = SystemTrayMenu::new();

    let mut scripts = scripts.to_owned();
    scripts.sort_by_key(|s| s.favorite);
    for script in scripts {
        if !script.running() {
            let start = CustomMenuItem::new(script.id.to_string(), format!("Start {}", script.name));
            starter_menu = starter_menu.add_item(start);
        } else {
            let stop = CustomMenuItem::new(script.id.to_string(), format!("Stop {}", script.name));
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptEnd {
    id: Uuid,
    status: i8,
    error: Option<String>,
}
