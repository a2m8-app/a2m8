use mlua::{Lua, Table};
use tokio::fs;

#[cfg(feature = "audio")]
mod audio;
#[cfg(feature = "clipboard")]
mod clipboard;
#[cfg(feature = "command")]
mod command;
#[cfg(feature = "displays")]
mod displays;
#[cfg(feature = "events")]
mod event_handler;
#[cfg(feature = "events")]
mod event_sender;
#[cfg(feature = "notify")]
mod notify;
mod sleep;
mod versions;

#[macro_export]
macro_rules! create_body {
    ($lua:expr, $($key:expr => $value:expr),*) => {
        {
            let tb = $lua.create_table()?;
            $(tb.set($key, $value)?;)*
            Ok(tb)
        }
    }
}

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

    let globals = lua.globals();

    //TODO(everyone): keep this sorted alphabetically
    #[rustfmt::skip]
    let result: Table = match module.as_str() {
#[cfg(feature = "audio")]       "audio" => audio::init(lua)?,
#[cfg(feature = "clipboard")]   "clipboard" => clipboard::init(lua)?,
#[cfg(feature = "command")]     "command" => command::init(lua)?,
#[cfg(feature = "displays")]    "displays" => displays::init(lua)?,
#[cfg(feature = "events")]      "event_handler_internal" => event_handler::init(lua)?,
#[cfg(feature = "events")]      "event_handler" => load_std().await?,
#[cfg(feature = "events")]      "event_sender" => event_sender::init(lua)?,
#[cfg(feature = "notify")]      "notify" => notify::init(lua)?,
/* always-on */                 "sleep" => sleep::init(lua)?,
/* always-on */                 "versions" => versions::init(lua)?,
#[cfg(feature = "events")]      "shortcuts" => load_std().await?,
/* always-on */                 "utils" => load_std().await?,
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
