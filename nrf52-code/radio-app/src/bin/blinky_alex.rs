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
    let led1 = board.leds._1;
    let led2 = board.leds._2;
    let led3 = board.leds._3;
    let led4 = board.leds._4;
    let mut leds = [led1, led2, led3, led4];
    let mut timer = board.timer;
    for led in leds.iter_mut() {
        led.toggle();
        timer.wait(Duration::from_millis(500));
        defmt::println!("LED toggled at {:?}", dk::uptime());
    }
    for led in leds.iter_mut().rev() {
        led.toggle();
        timer.wait(Duration::from_millis(500));
        defmt::println!("LED toggled at {:?}", dk::uptime());
    }
    dk::exit()
}