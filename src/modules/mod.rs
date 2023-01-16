use mlua::{Lua, Table};

use tokio::fs;

use self::{
    clipboard::Clipboard,
    command::{run_command, run_command_piped},
    displays::EasyDisplay,
    event_handler::EventHandler,
    sleep::sleep,
    versions::VersionInfo,
};

mod clipboard;
mod command;
mod displays;
mod event_handler;
mod sleep;
mod versions;

pub async fn require(lua: &Lua, module: String) -> mlua::Result<Table> {
    let loaded_modules = lua.globals().get::<_, Table>("__INTERNAL_LOADED_MODULES")?;
    if let Ok(table) = loaded_modules.get::<_, Table>(&*module) {
        return Ok(table);
    }
    /* loads the module from the filesystem this needs to be updated when released */
    let load_std = || async {
        let mut path = std::env::current_dir().unwrap();
        path.push("std");
        path.push(module.clone());
        path.set_extension("lua");
        let code = fs::read_to_string(&path).await?;
        let table: Table = lua.load(&code).call_async(()).await?;
        Ok::<_, mlua::Error>(table)
    };
    /* Creates a table */
    macro_rules! create_table {
        ($($key:expr => $value:expr),*) => {
            {
                let tb = lua.create_table()?;
                $(tb.set($key, $value)?;)*
                tb
            }
        }
    }

    let globals = lua.globals();

    let result: Table = match module.as_str() {
        "event_handler_internal" => {
            create_table! {
                "event_handler" => EventHandler {}
            }
        }
        "event_handler" => load_std().await?,
        "display" => {
            create_table! {
                "display" => EasyDisplay {}
            }
        }
        "versions" => {
            create_table! {
                "version_info" => VersionInfo {
                    version: format!(
                        "{} {} ({}) {}",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION"),
                        env!("GIT_HASH"),
                        env!("BUILD_TYPE")
                    ),
                }
            }
        }
        "clipboard" => {
            create_table! {
                "clipboard" => Clipboard {}
            }
        }
        "command" => {
            create_table! {
                "run_command" => lua.create_async_function(run_command)?,
                "run_command_piped" => lua.create_async_function(run_command_piped)?
            }
        }
        "sleep" => {
            create_table! {
                "sleep" => lua.create_async_function(sleep)?
            }
        }
        "utils" => load_std().await?,
        "shortcuts" => load_std().await?,
        _ => {
            /* early return so other modules can be cached */
            return globals
                .get::<_, mlua::Function>("require_ref")?
                .call_async(module)
                .await;
        }
    };

    loaded_modules.set(module, result.clone())?;

    Ok(result)
}
