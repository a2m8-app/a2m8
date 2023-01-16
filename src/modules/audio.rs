use std::io::BufReader;

use mlua::Lua;

// DO NOT MAKE A SEPARATE FUNCTION FOR THIS THINGS WILL BREAK!

pub fn play_audio_blocking(_: &Lua, fname: String) -> mlua::Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(fname).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.sleep_until_end();
    Ok(())
}
pub fn play_audio(_: &Lua, fname: String) -> mlua::Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open(fname).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    Ok(())
}
