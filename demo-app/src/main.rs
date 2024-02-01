//! Rust Demo for the nRF52840, running FreeRTOS

// SPDX-FileCopyrightText: Copyright (c) 2024 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

#![no_main]
#![no_std]

use byte_strings::c;
use cortex_m_rt::entry;
use defmt_rtt as _;
use nrf52840_hal::prelude::OutputPin;
use panic_probe as _;

static BUILD_SLUG: Option<&str> = option_env!("BUILD_SLUG");

const DEMO_STACK_SIZE: u16 = 1024;

#[entry]
fn main() -> ! {
    defmt::println!(
        "Hello, this is version {}!",
        BUILD_SLUG.unwrap_or("unknown")
    );

    let pp = nrf52840_hal::pac::Peripherals::take().unwrap();

    let clocks = nrf52840_hal::Clocks::new(pp.CLOCK);
    let clocks = clocks.enable_ext_hfosc();
    let clocks =
        clocks.set_lfclk_src_external(nrf52840_hal::clocks::LfOscConfiguration::NoExternalNoBypass);
    let clocks = clocks.start_lfclk();
    let _clocks = clocks.enable_ext_hfosc();

    let pins = nrf52840_hal::gpio::p0::Parts::new(pp.P0);
    let mut led = pins
        .p0_13
        .degrade()
        .into_push_pull_output(nrf52840_hal::gpio::Level::High);

    let _ = led.set_low();

    unsafe {
        freertos_sys::xTaskCreate(
            // the function to call
            Some(test_task),
            // what to call it (must be a C string)
            c!("Test").as_ptr(),
            // how many bytes of stack
            DEMO_STACK_SIZE,
            // an argument to pass
            core::ptr::null_mut(),
            // what priority to run it at
            freertos_sys::tskIDLE_PRIORITY + 1,
            // returned task handle
            core::ptr::null_mut(),
        );
    }

    defmt::println!("Entering FreeRTOS kernel...");
    unsafe {
        freertos_sys::vTaskStartScheduler();
    }

    panic!("Kernel exited");
}

/// A function to run in a thread
unsafe extern "C" fn test_task(params: *mut core::ffi::c_void) {
    defmt::println!("I am the test task! params = {}", params);
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
