use std::future::Future;

use mlua::Lua;
use tokio::fs;

use self::{
    clipboard::Clipboard, displays::EasyDisplay, event_handler::EventHandler, sleep::sleep, versions::VersionInfo, command::run_command,
};

mod clipboard;
mod displays;
mod event_handler;
mod sleep;
mod versions;
mod command;

pub async fn require(lua: &Lua, module: String) -> mlua::Result<()> {
    let load_std = || async {
        let mut path = std::env::current_dir().unwrap();
        path.push("std");
        path.push(module.clone());
        path.set_extension("lua");
        let code = fs::read_to_string(&path).await?;
        lua.load(&code).exec_async().await?;
        Ok::<(), mlua::Error>(())
    };
    let globals = lua.globals();

    match module.as_str() {
        "event_handler" => {
            globals.set("event_handler", EventHandler {})?;
            load_std().await?;
        }
        "display" => {
            globals.set("display", EasyDisplay {})?;
        }
        "versions" => {
            globals.set(
                "version_info",
                VersionInfo {
                    version: format!(
                        "{} {} ({}) {}",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION"),
                        env!("GIT_HASH"),
                        env!("BUILD_TYPE")
                    ),
                },
            )?;
        }
        "clipboard" => {
            globals.set("clipboard", Clipboard {})?;
        }
        "command" => {
            globals.set("run_command", lua.create_async_function(run_command)?)?;
        }
        "sleep" => {
            globals.set("sleep", lua.create_async_function(sleep)?)?;
            load_std().await?;
        }
        _ => {
            globals
                .get::<_, mlua::Function>("require_ref")?
                .call_async(module)
                .await?;
        }
    }

    Ok(())
}
