use std::{collections::HashMap, io::Cursor, path::PathBuf};

use mlua::{Lua, LuaSerdeExt, Table, UserData, Value};
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::create_body;

#[doc(hidden)]
pub fn init(lua: &Lua) -> mlua::Result<mlua::Table> {
    create_body! (lua,
        "download_file" => lua.create_async_function(download_file)?,
        "fetch_text" => lua.create_async_function(fetch_text)?,
        "request" => lua.create_async_function(request)?
    )
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct RequestInit {
    method: Option<String>,
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
    query: Option<HashMap<String, String>>,
    timeout: Option<u64>,
    basic_auth: Option<(String, String)>,
    bearer_auth: Option<String>,
    form: Option<HashMap<String, String>>,
}

impl UserData for RequestInit {}

pub async fn request<'lua>(lua: &'lua Lua, (url, data): (String, Option<Value<'lua>>)) -> mlua::Result<Table<'lua>> {
    let client = reqwest::Client::new();
    let data: RequestInit = if let Some(data) = data {
        lua.from_value(data)?
    } else {
        RequestInit::default()
    };
    let mut req = client.request(
        Method::from_bytes(data.method.unwrap_or("GET".to_owned()).as_bytes())
            .map_err(|x| mlua::Error::RuntimeError(format!("Invalid method: {x}")))?,
        url,
    );
    if let Some(body) = data.body {
        req = req.body(body);
    }
    if let Some(headers) = data.headers {
        for (key, value) in headers {
            req = req.header(key, value);
        }
    }
    if let Some(query) = data.query {
        for (key, value) in query {
            req = req.query(&[(key, value)]);
        }
    }
    if let Some(timeout) = data.timeout {
        req = req.timeout(std::time::Duration::from_millis(timeout));
    }
    if let Some((username, password)) = data.basic_auth {
        req = req.basic_auth(username, Some(password));
    }
    if let Some(token) = data.bearer_auth {
        req = req.bearer_auth(token);
    }
    if let Some(form) = data.form {
        req = req.form(&form);
    }

    let resp = req
        .send()
        .await
        .map_err(|x| mlua::Error::RuntimeError(format!("Error while sending request: {x}")))?;

    let table = lua.create_table()?;
    table.set("status", resp.status().as_u16())?;
    table.set(
        "headers",
        resp.headers()
            .clone()
            .iter()
            .map(|(key, value)| (key.as_str().to_string(), value.to_str().unwrap().to_string()))
            .collect::<HashMap<String, String>>(),
    )?;
    table.set(
        "text",
        resp.text()
            .await
            .map_err(|x| mlua::Error::RuntimeError(format!("Error while reading response: {x}")))?,
    )?;
    Ok(table)
}

pub async fn download_file(_: &Lua, (url, path): (String, String)) -> mlua::Result<bool> {
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
pub async fn fetch_text(_: &Lua, (method, url, body): (String, String, Option<String>)) -> mlua::Result<String> {
    let client = reqwest::Client::new();
    let mut req = client.request(
        Method::from_bytes(method.as_bytes()).map_err(|x| mlua::Error::RuntimeError(format!("Invalid method: {x}")))?,
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
