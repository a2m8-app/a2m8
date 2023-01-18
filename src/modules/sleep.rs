use mlua::{Error as LuaError, Function, Lua};
use tokio::{task::LocalSet, time};

use crate::create_body;

pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "sleep" => lua.create_async_function(sleep)?
    )
}

pub async fn sleep(_: &Lua, time: f64) -> Result<(), LuaError> {
    time::sleep(time::Duration::from_millis((time * 1000.0) as u64)).await;
    Ok(())
}
