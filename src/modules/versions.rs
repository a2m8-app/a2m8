use mlua::UserData;

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
