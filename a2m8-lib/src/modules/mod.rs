use mlua::{Lua, Table};
use rust_embed::RustEmbed;

#[cfg(feature = "audio")]
mod audio;
#[cfg(feature = "clipboard")]
mod clipboard;
#[cfg(feature = "command")]
mod command;
#[cfg(feature = "displays")]
mod displays;
mod env;
#[cfg(feature = "events")]
mod event_handler;
#[cfg(feature = "events")]
mod event_sender;
mod json;
mod log;
#[cfg(feature = "network")]
mod network;
#[cfg(feature = "notify")]
mod notify;
#[cfg(feature = "open")]
mod open;
mod utils;
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
#[derive(RustEmbed)]
#[folder = "src/std/"]
struct StdFiles;

impl StdFiles {
    pub fn get_lua_file(name: &str) -> Option<String> {
        std::str::from_utf8(Self::get(&format!("{name}.lua"))?.data.as_ref())
            .ok()
            .map(|x| x.to_string())
    }
}

pub async fn require(lua: &Lua, module: String) -> mlua::Result<Table> {
    let loaded_modules = lua.globals().get::<_, Table>("__INTERNAL_LOADED_MODULES")?;
    if let Ok(table) = loaded_modules.get::<_, Table>(&*module) {
        return Ok(table);
    }
    /* loads the module from the filesystem this needs to be updated when released */
    let load_std = || async {
        let code = StdFiles::get_lua_file(&module)
            .ok_or(mlua::Error::RuntimeError(format!("module {} not found", module).into()))?;

        let table: Table = lua.load(&code).set_name(&module)?.call_async(()).await?;
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
/* always-on */                 "utils_internal" => utils::init(lua)?,
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
