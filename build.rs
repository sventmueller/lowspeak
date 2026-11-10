//! Links clang's runtime library: with macOS 14 as the oldest system, whisper.cpp's Metal code checks the running
//! version through `__isPlatformVersionAtLeast`, which only that library has, and rustc links without it. It goes in
//! as a library, so that a program of another crate that uses this one gets it as well.

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
    let dir = Path::new(resources.trim()).join("lib/darwin");
    let runtime = dir.join("libclang_rt.osx.a");
    assert!(runtime.exists(), "{} is missing", runtime.display());
    // A library, unlike a link argument, also reaches the programs of crates that depend on this one.
    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=static=clang_rt.osx");
}
