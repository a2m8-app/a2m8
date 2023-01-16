use mlua::{Error as LuaError, Lua};

use crate::modules::require;

mod assets;
mod modules;
mod private;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), LuaError> {
    let lua = Lua::new();

    let globals = lua.globals();

    globals.set("require_ref", globals.get::<_, mlua::Function>("require")?)?;
    globals.set("require", lua.create_async_function(require)?)?;

    std::env::set_current_dir("./src").unwrap();
    lua.load(&std::fs::read_to_string("script.lua")?).exec_async().await?;
    Ok(())
}
