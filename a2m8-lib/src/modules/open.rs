use mlua::Lua;

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "open" => lua.create_function(open)?,
        "open_browser" => lua.create_function(open_browser)?
    )
}

pub fn open(_: &Lua, url: String) -> mlua::Result<()> {
    opener::open(url).map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
    Ok(())
}
pub fn open_browser(_: &Lua, url: String) -> mlua::Result<()> {
    opener::open_browser(url).map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
    Ok(())
}
