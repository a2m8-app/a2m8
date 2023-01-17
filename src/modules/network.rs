use std::{io::Cursor, path::PathBuf};

use mlua::Lua;
use reqwest::{Method, Url};

use crate::create_body;

pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "download_file" => lua.create_async_function(download_file)?,
        "fetch_text" => lua.create_async_function(fetch_text)?
    )
}
async fn download_file(_: &Lua, (url, path): (String, String)) -> mlua::Result<bool> {
    let resp = reqwest::get(url).await;
    if let Ok(resp) = resp {
        let mut content = Cursor::new(resp.bytes().await.unwrap());
        let file = std::fs::File::create(PathBuf::from(path));
        if file.is_err() {
            return Ok(false);
        }
        let mut file = file.unwrap();
        let res = std::io::copy(&mut content, &mut file);
        return Ok(res.is_ok());
    }
    Ok(false)
}
async fn fetch_text(_: &Lua, (method, url, body): (String, String, Option<String>)) -> mlua::Result<String> {
    let client = reqwest::Client::new();
    let mut req = client.request(
        Method::from_bytes(method.as_bytes())
            .map_err(|x| mlua::Error::RuntimeError(format!("Invalid method: {}", x)))?,
        url,
    );
    if let Some(body) = body {
        req = req.body(body);
    }
    let resp = req.send().await;

    if let Ok(resp) = resp {
        Ok(resp
            .text()
            .await
            .map_err(|_| mlua::Error::RuntimeError("Failed to download text".to_string()))?)
    } else {
        Err(mlua::Error::RuntimeError("Failed to download text".to_string()))
    }
}
