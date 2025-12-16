use std::fs;
use std::path::Path;
use winres::WindowsResource;

fn main() {
    #[cfg(windows)]
    {
        let mut res = WindowsResource::new();
        res.set_icon("assets/kenny.ico");
        res.compile().unwrap();
    }

    // Copy assets to the output directory
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);

    // Create assets directory in output
    let assets_dir = out_path.join("assets");
    fs::create_dir_all(&assets_dir).ok();

    // Copy font file
    if let Err(e) = fs::copy("assets/appFont.ttf", assets_dir.join("appFont.ttf")) {
        eprintln!("Warning: Failed to copy font file: {}", e);
    }

    println!("cargo:rerun-if-changed=assets/appFont.ttf");
}
