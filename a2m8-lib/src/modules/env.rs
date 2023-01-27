///! this module gives access to the environment variables directly
use mlua::Lua;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    let env = lua.create_table_from(std::env::vars())?;
    Ok(env)
}
