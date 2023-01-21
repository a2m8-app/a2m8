#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use directories::ProjectDirs;
use tauri::async_runtime::Mutex;

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let dirs = ProjectDirs::from("dev", "tricked", "A2M8").unwrap();
    let mut config = A2M8Config {
        scripts: Vec::new(),
        data_dir: dirs.data_dir().to_path_buf(),
    };
    config.load_scripts().await?;
    tauri::Builder::default()
        .manage(Mutex::new(config))
        .invoke_handler(tauri::generate_handler![
            create_script,
            update_script,
            delete_script,
            get_scripts,
            get_script
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
