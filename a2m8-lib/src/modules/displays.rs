use std::fs;

use mlua::{Lua, UserData};
use serde::{Deserialize, Serialize};

use crate::create_body;

pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "screens" => lua.create_function(screens)?,
        "screen_from_point" => lua.create_function(screen_from_point)?
    )
}

fn screens(_: &Lua, _: ()) -> mlua::Result<Vec<SerdeDisplayInfo>> {
    Ok(screenshots::DisplayInfo::all()
        .unwrap_or_default()
        .into_iter()
        .map(|x| {
            let x: SerdeDisplayInfo = x.into();
            x
        })
        .collect::<Vec<_>>())
}

fn screen_from_point(_: &Lua, (x, y): (i32, i32)) -> mlua::Result<SerdeDisplayInfo> {
    Ok(screenshots::DisplayInfo::from_point(x, y)
        .map(|x| {
            let x: SerdeDisplayInfo = x.into();
            x
        })
        .unwrap_or_default())
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
pub struct SerdeDisplayInfo {
    pub id: u32,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub rotation: f32,
    pub scale_factor: f32,
    pub is_primary: bool,
}

impl UserData for SerdeDisplayInfo {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        //create a simple macro to generate these methods
        macro_rules! add_field_method_get {
            ($name:ident) => {
                fields.add_field_method_get(stringify!($name), |_, this| Ok(this.$name));
            };
        }
        add_field_method_get!(id);
        add_field_method_get!(x);
        add_field_method_get!(y);
        add_field_method_get!(width);
        add_field_method_get!(height);
        add_field_method_get!(rotation);
        add_field_method_get!(scale_factor);
        add_field_method_get!(is_primary);
    }
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("capture", |_, this, outfile: String| {
            let display: screenshots::DisplayInfo = (*this).into();
            let screen = screenshots::Screen::new(&display);
            let image = screen.capture().unwrap();
            let buffer = image.buffer();
            fs::write(outfile, buffer).unwrap();
            Ok(())
        });
        methods.add_method(
            "capture_area",
            |_, this, (x, y, width, height, outfile): (i32, i32, u32, u32, String)| {
                let display: screenshots::DisplayInfo = (*this).into();
                let screen = screenshots::Screen::new(&display);
                let image = screen.capture_area(x, y, width, height).unwrap();
                let buffer = image.buffer();
                fs::write(outfile, buffer).unwrap();
                Ok(())
            },
        )
    }
}

impl From<screenshots::DisplayInfo> for SerdeDisplayInfo {
    fn from(info: screenshots::DisplayInfo) -> Self {
        SerdeDisplayInfo {
            id: info.id,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            rotation: info.rotation,
            scale_factor: info.scale_factor,
            is_primary: info.is_primary,
        }
    }
}
impl From<SerdeDisplayInfo> for screenshots::DisplayInfo {
    fn from(info: SerdeDisplayInfo) -> Self {
        screenshots::DisplayInfo {
            id: info.id,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            rotation: info.rotation,
            scale_factor: info.scale_factor,
            is_primary: info.is_primary,
        }
    }
}
