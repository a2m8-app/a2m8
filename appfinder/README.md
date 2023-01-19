# Appfinder

find the location of applications on the system.

Resolves multiple folders on windows to find binaries on

## Usage

```rust
use appfinder::get_app_path;
fn main() {
    let path = get_app_path("firefox");
    println!("Firefox is located at: {:?}", path);
}
```

```rust
use appfinder::get_app_path;
fn main() {
    let path = get_app_path("discord");
    println!("Discord is located at: {:?}", path);

}
```

## Os support

| thing         | Windows | Macos | Linux |
| ------------- | :-----: | :---: | :---: |
| path          |   🚫    |  ✅   |  ✅   |
| desktop files |   🚫    |  🚫   |  ✅   |
| start menu    |   ✅    |  🚫   |  🚫   |
| appdata       |   ✅    |  🚫   |  🚫   |

### Caveats

- on linux desktopfiles return the full commands even arguments these can be manually removed
- the name of the application you want to find must be lowercase
