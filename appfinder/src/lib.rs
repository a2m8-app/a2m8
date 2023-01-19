#![doc = include_str!("../README.md")]


cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub(crate) mod windows;
        pub(crate) use windows::*;
    } else if #[cfg(target_os = "linux")] {
        pub(crate) mod linux;
        pub(crate) use linux::*;
    } else if #[cfg(target_os = "macos")] {
        pub(crate) mod macos;
        pub(crate) use macos::*;
    } else {
        compile_error!("Unsupported OS");
    }
}

pub fn get_app_path(app_name: &str) -> Option<String> {
    _get_app_path(app_name)
}

pub fn remove_arguments(cmd: &str) -> String {
    cmd.split_whitespace().next().unwrap().to_string()
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_get_app_path() {
        assert_eq!(
            super::get_app_path("cat"),
            Some("/usr/bin/cat".to_string())
        );
    }
}