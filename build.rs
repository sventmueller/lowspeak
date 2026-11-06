//! Links clang's runtime library: with macOS 14 as the oldest system, whisper.cpp's Metal code checks the running
//! version through `__isPlatformVersionAtLeast`, which only that library has, and rustc links without it.

use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("macos") {
        return;
    }
    let out = Command::new("xcrun")
        .args(["clang", "--print-resource-dir"])
        .output()
        .expect("xcrun clang --print-resource-dir needs the Xcode command line tools");
    assert!(
        out.status.success(),
        "xcrun clang --print-resource-dir failed"
    );
    let resources = String::from_utf8(out.stdout).expect("the resource directory is UTF-8");
    let runtime = Path::new(resources.trim()).join("lib/darwin/libclang_rt.osx.a");
    assert!(runtime.exists(), "{} is missing", runtime.display());
    println!("cargo:rustc-link-arg={}", runtime.display());
}
