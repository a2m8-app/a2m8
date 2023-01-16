use std::thread;

use clipboard::Clipboard;
use displays::EasyDisplay;
use event_handler::EventHandler;
use mlua::{Error as LuaError, Lua};
use once_cell::sync::Lazy;
use rdev::{listen, Event};
use tokio::{sync::{
    mpsc::{self, UnboundedReceiver},
    Mutex,
}, fs};
use tokio::time;

mod clipboard;
mod displays;
mod event_handler;
mod sleep;
mod better_require;
mod event_listener;

use crate::sleep::sleep;
use crate::better_require::better_require;


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), LuaError> {
    let lua = Lua::new();

    let globals = lua.globals();

    globals.set("sleep", lua.create_async_function(sleep)?)?;
    globals.set("require_ref", globals.get::<_, mlua::Function>("require")?)?;
    globals.set("require", lua.create_async_function(better_require)?)?;

    std::env::set_current_dir("./src").unwrap();
    lua.load(&std::fs::read_to_string("script.lua")?).exec_async().await?;
    Ok(())
}
