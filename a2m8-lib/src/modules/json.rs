use crate::create_body;
use mlua::{Lua, LuaSerdeExt};

pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "parse" => lua.create_function(parse)?,
        "stringify" => lua.create_function(stringify)?
    )
}

fn parse(lua: &Lua, json: String) -> mlua::Result<mlua::Value> {
    let value: serde_json::Value = serde_json::from_str(&json).map_err(mlua::Error::external)?;
    let table = lua.to_value(&value)?;
    Ok(table)
}
fn stringify(lua: &Lua, (v, pretty): (mlua::Value, bool)) -> mlua::Result<String> {
    let value: serde_json::Value = lua.from_value(v)?;

    let json = if pretty {
        serde_json::to_string_pretty(&value)
    } else {
        serde_json::to_string(&value)
    }
    .map_err(mlua::Error::external)?;

    Ok(json)
}
