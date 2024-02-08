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

defmt::timestamp!("{=u32}", { unsafe { freertos_sys::xTaskGetTickCount() } });

#[entry]
fn main() -> ! {
    defmt::info!(
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

    // Create a dynamic (heap allocated) queue.
    //
    // We can treat this handle as having 'static lifetime.
    //
    // See FreeRTOSConfig.h for the default heap size.
    let queue_handle = unsafe {
        freertos_sys::xQueueGenericCreate(
            // number of items
            10,
            // size of each item
            1,
            // basic queue
            freertos_sys::_queueQUEUE_TYPE_BASE,
        )
    };

    // Create Task 1
    unsafe {
        freertos_sys::xTaskCreate(
            // the function to call
            Some(test_task1),
            // what to call it (must be a C string)
            c!("Test1").as_ptr(),
            // how many bytes of stack
            DEMO_STACK_SIZE,
            // an argument to pass
            queue_handle as *mut core::ffi::c_void,
            // what priority to run it at
            freertos_sys::tskIDLE_PRIORITY + 1,
            // returned task handle
            core::ptr::null_mut(),
        );
    }

    // Create Task 2
    unsafe {
        freertos_sys::xTaskCreate(
            // the function to call
            Some(test_task2),
            // what to call it (must be a C string)
            c!("Test2").as_ptr(),
            // how many bytes of stack
            DEMO_STACK_SIZE,
            // an argument to pass
            queue_handle as *mut core::ffi::c_void,
            // what priority to run it at
            freertos_sys::tskIDLE_PRIORITY + 1,
            // returned task handle
            core::ptr::null_mut(),
        );
    }

    defmt::info!("Entering FreeRTOS kernel...");
    unsafe {
        freertos_sys::vTaskStartScheduler();
    }

    panic!("Kernel exited");
}

/// A function to run in a thread
///
/// Counts from 0 to 10, posting the counter into a queue.
unsafe extern "C" fn test_task1(params: *mut core::ffi::c_void) {
    defmt::info!("params = {}", params);
    let queue = params as *mut freertos_sys::QueueDefinition;
    loop {
        for counter in 0u8..=10 {
            defmt::debug!("counter = {=u8}", counter);
            unsafe {
                freertos_sys::xQueueGenericSend(
                    queue,
                    &counter as *const u8 as *const core::ffi::c_void,
                    0,
                    freertos_sys::_queueSEND_TO_BACK,
                );
            }
            freertos_sys::vTaskDelay(1000 / freertos_sys::_portTICK_PERIOD_MS);
        }
    }
}

/// A second function to run in a thread
///
/// Waits on a queue and prints the number that comes out.
unsafe extern "C" fn test_task2(params: *mut core::ffi::c_void) {
    defmt::info!("params = {}", params);
    let queue = params as *mut freertos_sys::QueueDefinition;
    loop {
        let mut value = 0u8;
        // blocking queue read - zero indicates error/timeout
        let result = unsafe {
            freertos_sys::xQueueReceive(
                queue,
                &mut value as *mut u8 as *mut core::ffi::c_void,
                freertos_sys::_portMAX_DELAY,
            )
        };
        if result != 0 {
            defmt::info!("Got value in queue: {=u8}", value);
        } else {
            defmt::error!("Error reading from queue");
        }
    }
}

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
