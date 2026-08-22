use esp_hal::{
    delay::Delay,
    gpio::{Input, InputConfig, InputPin, Level, Output, OutputConfig, OutputPin},
    time::{Duration, Instant},
};

use crate::distance_cm;

pub struct Sonar<'a> {
    trigger_pin: Output<'a>,
    echo_pin: Input<'a>,
    delay: Delay,
}

impl<'a> Sonar<'a> {
    pub fn new(trigger_pin: impl OutputPin + 'a, echo_pin: impl InputPin + 'a) -> Self {
        let trigger_pin = Output::new(trigger_pin, Level::Low, OutputConfig::default());
        let echo_pin = Input::new(echo_pin, InputConfig::default());
        let delay = Delay::new();
        Sonar {
            trigger_pin,
            echo_pin,
            delay,
        }
    }

    /// Measures the distance to an object in centimeters.
    ///
    /// Returns `None` when the echo signal is not received before the timeout.
    pub fn distance(&mut self, iteration_delay: u32) -> Option<f32> {
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

        let duration_micros = (echo_end - echo_start).as_micros();
        let distance_cm = distance_cm(duration_micros as f32);

        self.delay.delay_millis(iteration_delay);

        Some(distance_cm)
    }
}
