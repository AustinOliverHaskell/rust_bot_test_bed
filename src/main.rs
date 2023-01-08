use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() {

    let gpio = Gpio::new().unwrap();

    let mut pin22 = gpio.get(22).unwrap().into_output();
    let mut pin23 = gpio.get(23).unwrap().into_output();
    let mut pin24 = gpio.get(24).unwrap().into_output();
    let mut pin25 = gpio.get(25).unwrap().into_output();

    loop {
        println!("Start Cycle");



        pin22.set_high();
        thread::sleep(Duration::from_millis(500));
        pin22.set_low();

        pin23.set_high();
        thread::sleep(Duration::from_millis(500));
        pin23.set_low();

        pin24.set_high();
        thread::sleep(Duration::from_millis(500));
        pin24.set_low();

        pin25.set_high();
        thread::sleep(Duration::from_millis(500));
        pin25.set_low();


        thread::sleep(Duration::from_millis(500));
    }
}
