use a2m8_lib::require;
use mlua::Lua;

fn add_globals(lua: &Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    globals.set("require_ref", globals.get::<_, mlua::Function>("require")?)?;
    globals.set("require", lua.create_async_function(require)?)?;
    globals.set("__INTERNAL_LOADED_MODULES", lua.create_table()?)?;

    Ok(())
}

pub async fn create_lua() -> mlua::Result<Lua> {
    let lua = Lua::new();
    add_globals(&lua)?;
    require(&lua, "preload".to_owned()).await?;
    Ok(lua)
}
