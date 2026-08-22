#![no_std]

#[cfg(target_arch = "riscv32")]
pub mod sonar;

pub fn distance_cm(micros: f32) -> f32 {
    micros / 58.31
}
