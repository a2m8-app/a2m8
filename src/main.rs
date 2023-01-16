use mlua::{Error as LuaError, Lua};

use crate::{better_require::better_require, sleep::sleep};

mod better_require;
mod clipboard;
mod displays;
mod event_handler;
mod event_listener;
mod sleep;
mod versions;

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
