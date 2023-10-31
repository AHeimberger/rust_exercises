#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

#[entry]
fn main() -> ! {
    // to enable more verbose logs, go to your `Cargo.toml` and set defmt logging levels
    // to `defmt-trace` by changing the `default = []` entry in `[features]`

    let board = dk::init().unwrap();
    let mut led_1 = board.leds._1;
    let mut led_2 = board.leds._2;
    let button_1 = board.buttons._1;
    let button_2 = board.buttons._2;
    let button_3 = board.buttons._3;
    let button_4 = board.buttons._4;
    let mut timer = board.timer;

    loop {
        if button_1.is_pressed() | button_2.is_pressed() | button_3.is_pressed() | button_4.is_pressed() {
            led_1.on();
            defmt::println!("Button Pressed");
        } else {
            led_1.off();
        }

        led_2.toggle();
        timer.wait(Duration::from_secs(1));
    }
}
