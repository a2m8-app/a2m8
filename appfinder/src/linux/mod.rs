use std::{fs, path::PathBuf};

use freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter};

pub fn _get_app_path(app_name: &str) -> Option<String> {
    //prefer bin over desktop entries since those are more likely to not include the full path
    let mut paths = std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .map(PathBuf::from)
        .collect::<Vec<_>>();
    paths.push(PathBuf::from("/usr/bin"));
    paths.push(PathBuf::from("/usr/local/bin"));
    paths.push(PathBuf::from("/bin"));
    for path in paths {
        if let Ok(reader) = fs::read_dir(&path) {
            for entry in reader.flat_map(|e| e) {
                let path = entry.path();
                if let Ok(metadata) = fs::metadata(&path) {
                    if metadata.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if file_name.to_string_lossy().to_lowercase() == app_name {
                                return Some(path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    for path in Iter::new(default_paths()) {
        if let Ok(bytes) = fs::read_to_string(&path) {
            if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                if entry.name(None).map(|x| x.to_lowercase()) == Some(app_name.to_owned())
                    || entry.appid.to_lowercase() == app_name
                {
                    if let Some(exec) = entry.exec() {
                        return Some(exec.to_string());
                    }
                }
            }
        }
    }
    None
}
