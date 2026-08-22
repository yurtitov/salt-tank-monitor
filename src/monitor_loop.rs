use defmt::{info, warn};

use crate::sonar::Sonar;

pub struct Monitor {
    pub sonar: Sonar<'static>,
}

impl Monitor {
    pub fn new(sonar: Sonar<'static>) -> Self {
        Monitor { sonar }
    }

    pub fn run(&mut self) -> ! {
        loop {
            match self.sonar.distance(1000) {
                Some(distance) => {
                    info!("Distance: {} cm", distance);
                }
                None => warn!("Timeout: Echo is not received"),
            }
        }
    }
}
