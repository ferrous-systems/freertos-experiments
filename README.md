# Rust on FreeRTOS Demo

This repository shows how to compile a Rust application which runs on FreeRTOS.

* `freertos-kernel` is a git submodule to the FreeRTOS github repo
* `freertos-sys` is a Rust library which runs bindgen on the FreeRTOS headers, and compiles the FreeRTOS source for Cortex-M4F
* `demo-app` is a Rust application which uses `freertos-sys`, which runs on the nRF52840DK

```console
$ git submodule update --init
$ cd demo-app
$ DEFMT_LOG=info cargo run --release
    Finished release [optimized + debuginfo] target(s) in 0.04s
     Running `probe-rs run --chip nRF52840_xxAA target/thumbv7em-none-eabihf/release/demo-app`
      Erasing ✔ [00:00:00] [############################################################################################################################] 12.00 KiB/12.00 KiB @ 31.93 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [############################################################################################################################] 12.00 KiB/12.00 KiB @ 39.68 KiB/s (eta 0s )    Finished in 0.699s
0 INFO  Hello, this is version unknown!
└─ demo_app::__cortex_m_rt_main @ src/main.rs:27  
0 INFO  Entering FreeRTOS kernel...
└─ demo_app::__cortex_m_rt_main @ src/main.rs:83  
0 INFO  test_task2(params = 0x0, counter = 0)
└─ demo_app::test_task2 @ src/main.rs:105 
0 INFO  test_task1(params = 0x0, counter = 0)
└─ demo_app::test_task1 @ src/main.rs:95  
50 INFO  test_task1(params = 0x0, counter = 1)
└─ demo_app::test_task1 @ src/main.rs:95  
100 INFO  test_task1(params = 0x0, counter = 2)
└─ demo_app::test_task1 @ src/main.rs:95  
```

## Prerequisites

FreeRTOS is downloaded in source form using a git submodule. Ensure you have fetched the submodule, e.g. with:

```sh
git submodule update --init
```

Because FreeRTOS is written in C, you will require a C compiler for your target. We compile the C code using the [`cc` crate](https://crates.io/crates/cc) which does a good job of finding an appropriate C compiler. You can download Arm's build of [GCC for arm-none-eabi](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads), or your system package manager might provide `gcc-arm-none-eabi` or `arm-none-eabi-gcc` packages.

## Licence

* Copyright (c) 2024 Ferrous Systems
* SPDX-License-Identifier: MIT OR Apache-2.0
