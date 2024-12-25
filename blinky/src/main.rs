use rppal::gpio::Gpio;
use std::thread;
use std::time::Duration;

fn main() {
    const GPIO_PIN: u8 = 14;

    let mut pin = Gpio::new()
        .expect("Failed to initialize GPIO")
        .get(GPIO_PIN)
        .expect("Failed to get GPIO pin")
        .into_output();

    loop {
        pin.set_high();
        thread::sleep(Duration::from_millis(500)); // Wait 500 ms
        pin.set_low();
        thread::sleep(Duration::from_millis(500)); // Wait 500 ms
    }
}
