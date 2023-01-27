use crate::create_body;
use mlua::{Lua, Table, UserData};

use crate::prelude::*;

pub fn init(lua: &Lua) -> mlua::Result<Table> {
    create_body!(
        lua,
        "version_info" => VersionInfo {
            version: format!(
                "{} {} ({}) {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("GIT_HASH"),
                env!("BUILD_TYPE")
            ),
        }
    )
}

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub(crate) version: String,
}

impl UserData for VersionInfo {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("version", |_, this| Ok(this.version.clone()));
        fields.add_field_function_get("lua", |lua, _| lua.globals().get::<_, String>("_VERSION"));
    }
}
