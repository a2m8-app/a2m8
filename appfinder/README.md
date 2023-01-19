# Appfinder

find the location of applications on the system.

## Usage

```rust
use appfinder::get_app_path;
fn main() {
    let path = get_app_path("firefox");
    println!("Firefox is located at: {:?}", path);
}
```

## Os support

| thing         | Windows | Macos | Linux |
| ------------- | ------- | ----- | ----- |
| path          | no      | yes   | yes   |
| desktop files | no      | no    | yes   |
| start menu    | yes     | no    | no    |
| appdata       | yes     | no    | no    |
| system apps   | yes     | yes   | yes   |

### Caveats

- on linux desktopfiles return the full commands even arguments these can be manually removed