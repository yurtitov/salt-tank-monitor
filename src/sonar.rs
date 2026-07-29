use esp_hal::{
    delay::Delay,
    gpio::{Input, Output},
    time::{Duration, Instant},
};

pub struct Sonar<'a> {
    trigger_pin: Output<'a>,
    echo_pin: Input<'a>,
    delay: Delay,
}

impl<'a> Sonar<'a> {
    pub fn from(trigger_pin: Output<'a>, echo_pin: Input<'a>) -> Self {
        let delay = Delay::new();
        Sonar {
            trigger_pin,
            echo_pin,
            delay,
        }
    }

    pub fn distance(&mut self) -> Option<f32> {
        self.trigger_pin.set_low();
        self.delay.delay_micros(2);
        self.trigger_pin.set_high();
        self.delay.delay_micros(10);
        self.trigger_pin.set_low();

        let wait_start = Instant::now();
        let mut got_echo = true;
        while self.echo_pin.is_low() {
            if wait_start.elapsed() > Duration::from_micros(30_000) {
                got_echo = false;
                break;
            }
        }

        if !got_echo {
            self.delay.delay_millis(200);
            return Option::None;
        }

        let echo_start = Instant::now();
        while self.echo_pin.is_high() {
            if echo_start.elapsed() > Duration::from_micros(30_000) {
                break;
            }
        }

        let echo_end = Instant::now();

        let duration_us = (echo_end - echo_start).as_micros();

        Some(duration_us as f32 / 58.31)
    }
}
