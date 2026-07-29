#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::{error, info, warn};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{self, Input, InputConfig, Output, OutputConfig};
use esp_hal::main;
use esp_hal::time::{Duration, Instant};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut led = Output::new(
        peripherals.GPIO8,
        gpio::Level::High,
        OutputConfig::default(),
    );

    let delay = Delay::new();

    let mut trig = Output::new(peripherals.GPIO7, gpio::Level::Low, OutputConfig::default());
    let echo = Input::new(peripherals.GPIO10, InputConfig::default());

    loop {
        led.set_high();

        trig.set_low();
        delay.delay_micros(2);
        trig.set_high();
        delay.delay_micros(10);
        trig.set_low();

        let wait_start = Instant::now();
        let mut got_echo = true;
        while echo.is_low() {
            if wait_start.elapsed() > Duration::from_micros(30_000) {
                got_echo = false;
                break;
            }
        }

        if !got_echo {
            warn!("Timeout: Echo is not received");
            delay.delay_millis(200);
            continue;
        }

        let echo_start = Instant::now();
        while echo.is_high() {
            if echo_start.elapsed() > Duration::from_micros(30_000) {
                break;
            }
        }

        let echo_end = Instant::now();

        let duration_us = (echo_end - echo_start).as_micros();

        let distance_cm = duration_us as f32 / 58.0;

        if distance_cm < 30.0 {
            led.set_low();
        } else {
            led.set_high();
        }

        info!("Distance: {} cm", distance_cm);

        delay.delay_millis(200);
    }

}
