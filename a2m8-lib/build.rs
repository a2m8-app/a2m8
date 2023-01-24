fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=BUILD_TYPE=unofficial");
    //set the git hash env variable
    if let Ok(git_hash) = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        let git_hash = String::from_utf8(git_hash.stdout).unwrap();
        println!("cargo:rustc-env=GIT_HASH={git_hash}");
    } else {
        println!("cargo:rustc-env=GIT_HASH=unknown");
    }
}
