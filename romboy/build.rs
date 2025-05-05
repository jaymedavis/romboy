use std::env;
use std::fs;
use std::path::{Path, PathBuf};

extern crate winresource;

fn main() {
    slint_build::compile("ui/main-window.slint").unwrap();

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        winresource::WindowsResource::new()
            .set_icon("assets/logo.ico")
            .compile().unwrap();
    }

    // determine the output directory (target/debug or target/release)
    let profile = env::var("PROFILE").unwrap(); // "debug" or "release"
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".into());
    let out_dir = PathBuf::from(target_dir).join(&profile);

    // copy settings.toml
    let src_settings = Path::new("settings.toml");
    let dest_settings = out_dir.join("settings.toml");
    fs::copy(&src_settings, &dest_settings).expect("Failed to copy settings.toml");

    // copy assets directory recursively
    let src_assets = Path::new("assets");
    let dest_assets = out_dir.join("assets");
    if src_assets.exists() {
        if dest_assets.exists() {
            fs::remove_dir_all(&dest_assets).expect("Failed to remove old assets directory");
        }
        
        copy_dir_all(&src_assets, &dest_assets).expect("Failed to copy assets directory");
    }
}

// helper function to recursively copy a directory
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
