use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=src/plugin.c");

    println!("cargo:rerun-if-env-changed=SURICATA_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=DEP_NDPI_INCLUDE");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let suricata_include_dir = env::var("SURICATA_INCLUDE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/usr/include/suricata"));
    let ndpi_include_dir = env::var("DEP_NDPI_INCLUDE")
        .map(PathBuf::from)
        .expect("missing DEP_NDPI_INCLUDE from ndpi-sys");

    if !suricata_include_dir.is_dir() {
        panic!(
            "Suricata include directory not found at '{}'. Set SURICATA_INCLUDE_DIR to override.",
            suricata_include_dir.display()
        );
    }
    let plugin_version = env::var("CARGO_PKG_VERSION").expect("missing CARGO_PKG_VERSION");
    let plugin_version_c = format!("\"{}\"", plugin_version);

    let mut build = cc::Build::new();
    build
        .file(manifest_dir.join("src/plugin.c"))
        .include(&suricata_include_dir)
        .include(&ndpi_include_dir)
        .flag_if_supported("-std=gnu11")
        .flag_if_supported("-Wno-unused-parameter")
        .define("NDPI_PLUGIN_VERSION", Some(plugin_version_c.as_str()));
    build.compile("plugin_shim");

    // We intentionally keep Rust minimal, so we force native link flags here.
    // `plugin.c` calls into nDPI, therefore the final cdylib must link libndpi.a.
    println!("cargo:rustc-link-lib=static=ndpi");

    // nDPI uses libm on non-Windows targets.
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() != "windows" {
        println!("cargo:rustc-link-lib=m");
    }
}
