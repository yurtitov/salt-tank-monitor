#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

mod sonar;

use defmt::{error, info, warn};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{self, Output, OutputConfig};
use esp_hal::main;

use crate::sonar::Sonar;

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

    let mut sonar = Sonar::new(peripherals.GPIO7, peripherals.GPIO10);

    loop {
        led.set_high();

        match sonar.distance() {
            Some(distance) => {
                info!("Distance: {} cm", distance);
                if distance < 30.0 {
                    led.set_low();
                } else {
                    led.set_high();
                }
            }
            None => warn!("Timeout: Echo is not received"),
        }
    }
}
