use tauri_build::Attributes;

fn main() {
    println!("{:?}", tauri_build::try_build(Attributes::default()));
}
