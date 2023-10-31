// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry; 
// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use radio_app as _;

// the custom entry point 👇🏾
// - macro takes te function and hides it
#[entry]
fn main() -> ! {
    //      ˆˆˆ
    //       ! is the 'never' type: this function never returns

    // initializes the peripherals
    // - board support package
    // - returns error if e.g. called twice
    dk::init().unwrap();

    // drops the println messages into some memory which is constantly
    // looked up be the pc (probe-run)
    defmt::println!("Hello, world!"); // 👋🏾

    loop {
        // breakpoint: halts the program's execution
        asm::bkpt();
    }
}
