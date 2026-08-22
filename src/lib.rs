#![no_std]

#[cfg(target_arch = "riscv32")]
pub mod sonar;

#[cfg(target_arch = "riscv32")]
pub mod monitor_loop;

pub fn distance_cm(micros: f32) -> f32 {
    micros / 58.31
}
