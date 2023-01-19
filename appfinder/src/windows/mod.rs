use std::{error::Error, fs, path::PathBuf};

fn get_search_paths() -> Vec<PathBuf> {
    let programdata = std::env::var("ProgramData").ok();
    let appdata = std::env::var("APPDATA").ok();
    let programfiles = std::env::var("ProgramFiles").ok();
    let programfiles_x86 = std::env::var("ProgramFiles(x86)").ok();
    let windir = std::env::var("windir").ok();
    let localappdata = std::env::var("LOCALAPPDATA").ok();

    let mut paths = vec![];

    if let Some(path) = programdata {
        paths.push(PathBuf::from(path + "\\Microsoft\\Windows\\Start Menu\\Programs"));
    }

    if let Some(path) = appdata {
        paths.push(PathBuf::from(path + "\\Microsoft\\Windows\\Start Menu\\Programs"));
    }

    if let Some(path) = programfiles {
        paths.push(PathBuf::from(path));
    }

    if let Some(path) = programfiles_x86 {
        paths.push(PathBuf::from(path));
    }

    if let Some(path) = windir {
        paths.push(PathBuf::from(path + "\\System32"));
    }

    if let Some(path) = localappdata {
        paths.push(PathBuf::from(path));
    }
    paths
}

pub fn _get_app_path(app_name: &str) -> Option<String> {
    let mut matched = None;
    let mut paths = get_search_paths();
    while let Some(path) = paths.pop() {
        for entry in fs::read_dir(path)?.flat_map(|e| e) {
            let path = entry.path();
            if path.is_dir() {
                paths.push(path);
            } else {
                let file_name = path.file_name();
                if let Some(file_name) = file_name {
                    if file_name.to_string_lossy().to_lowercase() == app_name  || file_name.to_string_lossy().to_lowercase() == app_name.to_string() + ".exe"{
                        if entry.metadata()?.is_symlink() {
                            if let Ok(target) = entry.path().read_link() {
                                if target.is_file() {
                                    matched = Some(target);
                                    break;
                                }
                            }
                        } else  if entry.metadata()?.is_file() {
                            matched = Some(path);
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(matched.map(|path| path.as_os_str().to_string_lossy().to_string()))
}
