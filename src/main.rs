use displays::EasyDisplay;
use mlua::{Error as LuaError, Lua};

mod displays;
mod event_handler;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), LuaError> {
    let lua = Lua::new();
    // let screenshot = lua.create_function(|_, ()| -> Result<(), LuaError> {
    //     return 1;
    // })?;
    // lua.globals().set("rust_func", f)?;
    lua.globals().set("display", EasyDisplay {})?;

    lua.load(include_str!("./script.lua")).exec_async().await?;
    Ok(())
}
