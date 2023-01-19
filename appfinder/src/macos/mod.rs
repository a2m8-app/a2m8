pub fn _get_app_path(app_name: &str) -> Option<String> {
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
    None
}
