use std::fs::read_to_string;

use mlua::{Function, Lua, Table};
use rust_embed::RustEmbed;

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "command")]
pub mod command;
#[cfg(feature = "displays")]
pub mod displays;
pub mod env;
#[cfg(feature = "events")]
pub mod event_handler;
#[cfg(feature = "events")]
pub mod event_sender;
pub mod json;
pub mod log;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "notify")]
pub mod notify;
#[cfg(feature = "open")]
pub mod open;
pub mod utils;
pub mod versions;

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
#[derive(RustEmbed)]
#[folder = "src/std/"]
struct StdFiles;

impl StdFiles {
    pub fn get_lua_file(name: &str) -> Option<String> {
        std::str::from_utf8(Self::get(&format!("{name}.lua"))?.data.as_ref())
            .map(|x| x.to_string())
            .ok()
    }
}

pub async fn require(lua: &Lua, module: String) -> mlua::Result<Table> {
    let loaded_modules = lua.globals().get::<_, Table>("__INTERNAL_LOADED_MODULES")?;

    if let Ok(table) = loaded_modules.get::<_, Table>(&*module) {
        return Ok(table);
    }
    /* loads the module from the filesystem this needs to be updated when released */
    let load_std = || async {
        let code =
            StdFiles::get_lua_file(&module).ok_or(mlua::Error::RuntimeError(format!("module {module} not found")))?;

        let table: Table = lua.load(&code).set_name(&module)?.call_async(()).await?;
        Ok::<_, mlua::Error>(table)
    };

    let load_teal = || async {
        let table: Table = lua.load(tl::TEAL_LUA).set_name(&module)?.call_async(()).await?;
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
/* always-on */                 "env" => env::init(lua)?,
#[cfg(feature = "events")]      "event_handler_internal" => event_handler::init(lua)?,
#[cfg(feature = "events")]      "event_handler" => load_std().await?,
#[cfg(feature = "events")]      "event_sender_internal" => event_sender::init(lua)?,
#[cfg(feature = "events")]      "event_sender" => load_std().await?,
/* always-on */                 "json" => json::init(lua)?,
/* always-on */                 "log" => log::init(lua)?,
#[cfg(feature = "network")]     "network" => network::init(lua)?,
#[cfg(feature = "notify")]      "notify" => notify::init(lua)?,
#[cfg(feature = "open")]        "open" => open::init(lua)?,
/* always-on */                 "tl" => load_teal().await?,
/* always-on */                 "utils_internal" => utils::init(lua)?,
/* always-on */                 "versions" => versions::init(lua)?,
#[cfg(feature = "events")]      "shortcuts" => load_std().await?,
/* always-on */                 "utils" => load_std().await?,
/* always-on */                 m if m.ends_with(".tl")  => {
                                    let tl = if let Ok(table) = loaded_modules.get::<_, Table>("tl") {
                                        table
                                    } else {
                                        load_teal().await?
                                    };
                                    let load = tl.get::<_, Function>("load")?;
                                    let data = read_to_string(m)?;
                                    load.call::<_, Function>((data, m))?.call::<_, Table>(())?
                                }
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
