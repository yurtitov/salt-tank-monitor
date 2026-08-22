# Development

## Prerequisites

Install the Rust toolchain and the ESP32-C3 target:

```bash
rustup toolchain install 1.97.1
rustup target add riscv32imc-unknown-none-elf --toolchain 1.97.1
```

To flash and run the firmware, install `probe-rs` and connect an ESP32-C3 board through a supported debug probe. The probe must be connected before running the firmware command.

## Run tests

Unit tests and integration tests run on the host computer. They must not use the ESP32 target:

```bash
cargo test
```

Run only the integration test for the distance calculation:

```bash
cargo test --test distance
```

Integration tests are stored in `tests/`. Code that does not require GPIO, timers, or other ESP32 peripherals should be kept in the library so it can be tested this way.

## Build the firmware

Build the firmware for the ESP32-C3 target:

```bash
cargo build \
  --features firmware \
  --target riscv32imc-unknown-none-elf
```

For an optimized release build:

```bash
cargo build \
  --release \
  --features firmware \
  --target riscv32imc-unknown-none-elf
```

## Flash and run

The `runner` in `.cargo/config.toml` is configured to use `probe-rs` with an ESP32-C3. Build and flash the debug firmware with:

```bash
cargo run \
  --features firmware \
  --target riscv32imc-unknown-none-elf
```

For release firmware:

```bash
cargo run \
  --release \
  --features firmware \
  --target riscv32imc-unknown-none-elf
```

`cargo run` builds the firmware, then invokes the configured `probe-rs` runner to flash it and display its RTT/defmt output. Stop the running program with `Ctrl+C`.

## VS Code

The default Cargo test command can be used for host tests. A firmware build task must include both `--features firmware` and `--target riscv32imc-unknown-none-elf`; otherwise Cargo will not build the ESP32 binary.
