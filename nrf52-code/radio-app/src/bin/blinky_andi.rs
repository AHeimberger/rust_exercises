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
    let mut timer = board.timer;

    let mut my_leds = [ board.leds._1, board.leds._2, board.leds._4, board.leds._3 ];

    for _ in 0..10 {
        for led in my_leds.iter_mut() {
            led.toggle();
            timer.wait(Duration::from_millis(100));
            led.toggle();
            timer.wait(Duration::from_millis(10));
            // timer.wait(200.millis()); // some people implemented this
        }
    }

    dk::exit()
}
