///!
use mlua::{Lua, Table};

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<Table> {
    create_body!(
        lua,
        // "version"=> format!("{} {} ({}) {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("GIT_HASH"), env!("BUILD_TYPE") ),
        "lua" => lua.globals().get::<_, String>("_VERSION")?
    )
}
