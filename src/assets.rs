use std::{borrow::Cow, };

use rust_embed::RustEmbed;
use tokio::fs;

// pub async fn get_module(name: &'static str) -> String {
//     fs::read_to_string(format!("std/{}.lua", name)).await.unwrap()
// }