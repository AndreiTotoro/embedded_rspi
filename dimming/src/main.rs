use rppal::gpio::Gpio;
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    const GPIO_PIN: u8 = 14;

    let gpio = Gpio::new()?;
    let mut pin = gpio.get(GPIO_PIN)?.into_output();

    loop {
        for duty in 0..=100 {
            pin.set_pwm_frequency(1000.0, duty as f64 / 100.0)?;
            thread::sleep(Duration::from_millis(10));
        }

        // Gradually decrease brightness from 100% to 0%
        for duty in 0..=100 {
            pin.set_pwm_frequency(1000.0, (100 - duty) as f64 / 100.0)?;
            thread::sleep(Duration::from_millis(10));
        }
    }
}
