use std::io::BufReader;

use mlua::Lua;

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body!(lua,
        "play_audio_blocking" => lua.create_function(play_audio_blocking)?,
        "play_audio" => lua.create_function(play_audio)?
    )
}

// DO NOT MAKE A SEPARATE FUNCTION FOR THIS THINGS WILL BREAK!

/// plays audio from a file
pub fn play_audio_blocking(_: &Lua, fname: String) -> mlua::Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(fname).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
    Ok(())
}

/// plays audio from a file
pub fn play_audio(_: &Lua, fname: String) -> mlua::Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(fname).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    Ok(())
}
