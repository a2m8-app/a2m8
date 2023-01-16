use mlua::{Lua, Error as LuaError};
use tokio::time;

pub async fn sleep(_: &Lua, time: f64) -> Result<(), LuaError> {
    time::sleep(time::Duration::from_millis((time * 1000.0) as u64)).await;
    Ok(())
}