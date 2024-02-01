//! Build Script for freertos-sys
//!
//! Calls out to bindgen to generate a Rust crate from the FreeRTOS header
//! files. This is currently hard-coded for Cortex-M4F support.

// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let freertos_path = PathBuf::from("../freertos-kernel");

    let maybe_path = std::env::var("FREERTOS_SYS_CONFIG_DIR");
    // If they don't give us one, use our example config
    let config_path: PathBuf = maybe_path.as_deref().unwrap_or("example").into();

    build_headers(&freertos_path, &config_path);
    build_library(&freertos_path, &config_path);
}

/// Convert the FreeRTOS header file into a Rust crate
fn build_headers(freertos_path: &Path, config_path: &Path) {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Point to FreeRTOS headers
        .clang_arg(format!("-I{}", freertos_path.join("include").display()))
        .clang_arg(format!(
            "-I{}",
            freertos_path.join("portable/GCC/ARM_CM4F").display()
        ))
        .clang_arg(format!("-I{}", config_path.display()))
        // Add some custom headers
        .clang_arg("-I./include")
        // Disable standard includes (they belong to the host)
        .clang_arg("-nostdinc")
        // Set the target
        .clang_arg("--target=arm")
        .clang_arg("-mthumb")
        .clang_arg("-mcpu=cortex-m4")
        // Use hardfp
        .clang_arg("-mfloat-abi=hard")
        // We're no_std
        .use_core()
        // Format the output
        .formatter(bindgen::Formatter::Rustfmt)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let rust_source = bindings.to_string();

    let bindings_out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    std::fs::write(bindings_out_path, rust_source).expect("Couldn't write updated bindgen output");

    println!("cargo:rerun-if-changed=wrapper.h");
}

/// Compile the FreeRTOS kernel and link it
fn build_library(freertos_path: &Path, config_path: &Path) {
    cc::Build::new()
        .include(freertos_path.join("include"))
        .include(freertos_path.join("portable/GCC/ARM_CM4F"))
        .include(config_path)
        .opt_level_str("s")
        .file(freertos_path.join("portable/GCC/ARM_CM4F/port.c"))
        .file(freertos_path.join("portable/MemMang/heap_4.c"))
        .file(freertos_path.join("croutine.c"))
        .file(freertos_path.join("queue.c"))
        .file(freertos_path.join("timers.c"))
        .file(freertos_path.join("event_groups.c"))
        .file(freertos_path.join("stream_buffer.c"))
        .file(freertos_path.join("list.c"))
        .file(freertos_path.join("tasks.c"))
        .compile("freertos");
    println!(
        "cargo:rerun-if-changed={}",
        config_path.join("FreeRTOSConfig.h").display()
    );
}
