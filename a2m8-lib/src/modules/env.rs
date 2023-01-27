use mlua::Lua;

pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    let env = lua.create_table_from(std::env::vars())?;
    Ok(env)
}