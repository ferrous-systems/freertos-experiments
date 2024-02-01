# Rust on FreeRTOS Demo

This repository shows how to compile a Rust application which runs on FreeRTOS.

* `freertos-kernel` is a git submodule to the FreeRTOS github repo
* `freertos-sys` is a Rust library which runs bindgen on the FreeRTOS headers, and compiles the FreeRTOS source for Cortex-M4F
* `demo-app` is a Rust application which uses `freertos-sys`, which runs on the nRF52840DK

```console
$ git submodule update --init
$ cd demo-app
$ cargo run --release
   Compiling freertos-sys v0.1.0 (/Users/jonathan/Documents/ferrous-systems/freertos-experiments/freertos-sys)
   Compiling demo-app v0.0.0 (/Users/jonathan/Documents/ferrous-systems/freertos-experiments/demo-app)
    Finished release [optimized + debuginfo] target(s) in 2.43s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/demo-app`
      Erasing ✔ [00:00:00] [###############################################] 12.00 KiB/12.00 KiB @ 32.70 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [###############################################] 12.00 KiB/12.00 KiB @ 42.13 KiB/s (eta 0s )    Finished in 0.672s
Hello, this is version unknown!
Entering FreeRTOS kernel...
I am the test task! params = 0x0
```

## Licence

* Copyright (c) 2024 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
