use rand::Rng;
use rppal::gpio::Gpio;
use std::{error::Error, thread, time::Duration};

const REDPIN: u8 = 14;
const GREENPIN: u8 = 15;
const BLUENPIN: u8 = 18;

fn random_shade() -> f64 {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen_range(0.0..1.0);
    random_number
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut redpin = Gpio::new()?.get(REDPIN)?.into_output();
    let mut greenpin = Gpio::new()?.get(GREENPIN)?.into_output();
    let mut bluepin = Gpio::new()?.get(BLUENPIN)?.into_output();

    loop {
        redpin.set_pwm_frequency(1000.0, random_shade())?;
        greenpin.set_pwm_frequency(1000.0, random_shade())?;
        bluepin.set_pwm_frequency(1000.0, random_shade())?;
        thread::sleep(Duration::from_millis(500));

        redpin.set_pwm_frequency(1000.0, 0.0)?;
        greenpin.set_pwm_frequency(1000.0, 0.0)?;
        bluepin.set_pwm_frequency(1000.0, 0.0)?;
        thread::sleep(Duration::from_millis(500));
    }
}
