//! Build script for the Rust/FreeRTOS demo

// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    // put memory layout (linker script) in the linker search path
    fs::copy("memory.x", out_dir.join("memory.x"))?;
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");

    Ok(())
}
