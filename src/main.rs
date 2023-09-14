use esp_idf_hal::{
    delay::FreeRtos,
    gpio::{IOPin, PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_println::println;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("Starting 1-output\nThis application adds a button to control an LED blinking on and off every 1 second.\n");

    // Get all the peripherals
    let peripherals = Peripherals::take().unwrap();
    // Initialize Pin 8 as an output to drive the LED

    // use esp_idf_hal::peripherals::Peripherals::peripherals::*;
    // for x in [peripherals.pins.gpio0, peripherals.pins.gpio1 , ] {
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio8).unwrap();
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio7).unwrap();
/*     let mut led_pin = PinDriver::output(peripherals.pins.gpio6).unwrap(); */
    /* led_pin.set_low().unwrap(); */
    /* let mut led_pin = PinDriver::output(peripherals.pins.gpio5).unwrap(); */
    /* led_pin.set_low().unwrap(); */
    /* let mut led_pin = PinDriver::output(peripherals.pins.gpio4).unwrap(); */
    /* led_pin.set_low().unwrap(); */
    /* let mut led_pin = PinDriver::output(peripherals.pins.gpio3).unwrap(); */
    /* led_pin.set_low().unwrap(); */
    /* let mut led_pin = PinDriver::output(peripherals.pins.gpio2).unwrap(); */
    /* led_pin.set_low().unwrap(); */
    /* let mut led_pin = PinDriver::output(peripherals.pins.gpio1).unwrap(); */
    /* led_pin.set_low().unwrap(); */
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio0).unwrap();
    // led_pin.set_low().unwrap();
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio10).unwrap();
    // led_pin.set_low().unwrap();
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio18).unwrap();
    // led_pin.set_low().unwrap();
    let mut led_pin = PinDriver::output(peripherals.pins.gpio19).unwrap();
    led_pin.set_low().unwrap();
    // let mut led_pin = PinDriver::output(peripherals.pins.gpio9).unwrap();
    // led_pin.set_high().unwrap();

    // Downgrading the pin allows us to set the pull-down resistor
    let mut btn_pin = PinDriver::input(peripherals.pins.gpio7.downgrade()).unwrap();
    btn_pin.set_pull(Pull::Down).unwrap();

    let mut button_state = btn_pin.is_high();
    loop {
        FreeRtos::delay_ms(50);
        if btn_pin.is_high() && !button_state {
            println!("ON");
            button_state = true;
            led_pin.set_high().unwrap();
        } else if btn_pin.is_low() && button_state {
            println!("OFF");
            button_state = false;
            led_pin.set_low().unwrap();
        }
    }
}
