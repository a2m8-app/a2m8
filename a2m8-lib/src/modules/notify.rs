use mlua::{Lua, UserData};
use notify_rust::Notification;

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "new" => lua.create_function(EasyNotification::new_lua)?
    )
}

#[derive(Debug, Clone)]
pub struct EasyNotification(Notification);
impl EasyNotification {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self(Notification::new())
    }
    pub fn new_lua(_: &Lua, _: ()) -> mlua::Result<Self> {
        Ok(Self(Notification::new()))
    }
}

impl Default for EasyNotification {
    fn default() -> Self {
        Self::new()
    }
}

impl UserData for EasyNotification {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("appname", |_, this, appname: String| {
            this.0.appname(&appname);
            Ok(this.clone())
        });
        methods.add_method_mut("summary", |_, this, summary: String| {
            this.0.summary(&summary);
            Ok(this.clone())
        });
        methods.add_method_mut("subtitle", |_, this, subtitle: String| {
            this.0.subtitle(&subtitle);
            Ok(this.clone())
        });
        methods.add_method_mut("body", |_, this, body: String| {
            this.0.body(&body);
            Ok(this.clone())
        });
        methods.add_method_mut("icon", |_, this, icon: String| {
            this.0.icon(&icon);
            Ok(this.clone())
        });
        methods.add_method_mut("show", |_, this, ()| {
            //TODO: do something with the handle
            this.0.show().map_err(|x| mlua::Error::RuntimeError(x.to_string()))?;
            Ok(this.clone())
        });
    }
}
