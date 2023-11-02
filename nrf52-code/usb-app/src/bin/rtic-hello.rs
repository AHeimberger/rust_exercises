#![no_main]
#![no_std]

// rtic is an interrupt based system
// problem of deadlock 
// - shared resources will be locked and rtic will care about locking
// - local resources can be used in the created tasks and if a local resource
//   is used in two tasks rtic will say no
// on every compile rtic-expansiong.rs is generated from the rtic file
// - share data between main thread and task

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use cortex_m::asm;

    #[local]
    struct MyLocalResources {}

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        dk::init().unwrap();

        defmt::println!("Hello");
        (
            MySharedResources {},
            MyLocalResources {},
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::println!("world!");

        loop {
            asm::bkpt();
        }
    }
}
