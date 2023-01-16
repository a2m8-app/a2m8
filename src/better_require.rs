use crate::{clipboard::Clipboard, displays::EasyDisplay, event_handler::EventHandler, versions::VersionInfo};
use mlua::{Error as LuaError, Lua};
use tokio::fs;

pub async fn better_require(lua: &Lua, module: String) -> Result<(), LuaError> {
    match module.as_str() {
        "event_handler" => {
            lua.globals().set("event_handler", EventHandler {})?;
        }
        "display" => {
            lua.globals().set("display", EasyDisplay {})?;
            return Ok(());
        }
        "versions" => {
            lua.globals().set("version_info", VersionInfo {
                version: format!("{} {} ({}) {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("GIT_HASH"), env!("BUILD_TYPE")),
            })?;
            return Ok(());
        }
        "clipboard" => {
            lua.globals().set("clipboard", Clipboard {})?;
            return Ok(());
        }
        _ => {}
    }

    let mut path = std::env::current_dir().unwrap();
    path.push("std");
    path.push(module.clone());
    path.set_extension("lua");

    if let Ok(code) = fs::read_to_string(path).await {
        lua.load(&code).exec_async().await?;
        Ok(())
    } else {
        lua.globals()
            .get::<_, mlua::Function>("require_ref")?
            .call_async(module)
            .await?;
        Ok(())
    }
}
