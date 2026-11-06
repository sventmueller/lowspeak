//! The app icon.

use std::process::{Command, Stdio};

use anyhow::{Result, ensure};

use crate::*;

/// `assets/icon-1024.png` is `assets/icon.svg` rendered by resvg (`resvg -w 1024 -h 1024`) and must keep Apple's
/// macOS icon template outline.
pub(crate) fn icon() -> Result<()> {
    let root = root();
    let source = root.join("assets").join("icon-1024.png");
    ensure!(source.exists(), "{} is missing", source.display());
    let iconset = root.join("target").join("AppIcon.iconset");
    if iconset.exists() {
        std::fs::remove_dir_all(&iconset)?;
    }
    std::fs::create_dir_all(&iconset)?;
    for size in [16u32, 32, 128, 256, 512] {
        for (scale, suffix) in [(1, ""), (2, "@2x")] {
            let pixels = (size * scale).to_string();
            run(Command::new("sips")
                .args(["-z", &pixels, &pixels])
                .arg(&source)
                .arg("--out")
                .arg(iconset.join(format!("icon_{size}x{size}{suffix}.png")))
                .stdout(Stdio::null()))?;
        }
    }
    let target = root.join("assets").join("AppIcon.icns");
    run(Command::new("iconutil")
        .args(["-c", "icns"])
        .arg(&iconset)
        .arg("-o")
        .arg(&target))?;
    std::fs::remove_dir_all(&iconset)?;
    println!("Icon: {}", target.display());
    // Markdown cannot size an image, so the README shows one drawn at the size it should appear. Apple's template
    // places the shape at 100 px with a transparent margin, which would indent the icon; its shadow reaches to 82 px
    // at the sides and 94 px at the top.
    let readme = root.join("assets").join("readme-icon.png");
    run(Command::new("sips")
        .args([
            "--cropToHeightWidth",
            "860",
            "860",
            "--cropOffset",
            "94",
            "82",
        ])
        .arg(&source)
        .arg("--out")
        .arg(&readme)
        .stdout(Stdio::null()))?;
    run(Command::new("sips")
        .args(["-z", "168", "168"])
        .arg(&readme)
        .stdout(Stdio::null()))
}
