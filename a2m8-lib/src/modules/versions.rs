///!
use mlua::{Lua, Table};

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<Table> {
    create_body!(
        lua,
        "a2m8" => env!("CARGO_PKG_VERSION"),
        "lua" => lua.globals().get::<_, String>("_VERSION")?
    )
}
