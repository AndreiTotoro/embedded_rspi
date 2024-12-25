use std::{error::Error, thread, time::Duration};

use rppal::gpio::Gpio;

const REDPIN: u8 = 14;
const GREENPIN: u8 = 15;
const BLUENPIN: u8 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    let mut redpin = Gpio::new()?.get(REDPIN)?.into_output();
    let mut greenpin = Gpio::new()?.get(GREENPIN)?.into_output();
    let mut bluepin = Gpio::new()?.get(BLUENPIN)?.into_output();

    loop {
        bluepin.set_high();
        redpin.set_high();
        thread::sleep(Duration::from_millis(500));

        bluepin.set_low();
        redpin.set_low();
        thread::sleep(Duration::from_millis(500));

        greenpin.set_high();
        redpin.set_high();
        thread::sleep(Duration::from_millis(500));

        greenpin.set_low();
        redpin.set_low();
        thread::sleep(Duration::from_millis(500));
    }
    Ok(())
}
