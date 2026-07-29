# salt-tank-monitor

An IoT sensor designed for automatic salt level monitoring in a water softener brine tank for a private house.

## Features

* **Hardware:** Built on top of the **ESP32** microcontroller.
* **Firmware:** Written entirely in **Rust** (using the ESP-IDF / embedded-hal ecosystem) for bare-metal stability, memory safety, and high efficiency.
* **Goal:** Provides timely notifications when it is time to refill the water softener with salt pellets.

## Tech Stack

* **Language:** Rust (Current Stable)
* **Platform:** ESP32 (Xtensa / RISC-V depending on the exact chip variant)
* **Peripherals:** [e.g., Ultrasonic HC-SR04 / Time-of-Flight VL53L0X sensor]