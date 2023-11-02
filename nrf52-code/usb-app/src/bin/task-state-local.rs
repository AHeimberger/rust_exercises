#![no_main]
#![no_std]

// this imports `src/lib.rs`to retrieve our global logger + panicking-behavior
use usb_app as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use cortex_m::asm;
    use dk::peripheral::POWER;

    #[local]
    struct MyLocalResources {
        power: POWER,           // this time power as local resource
        counter: u8
    }

    #[shared]
    struct MySharedResources {
    }

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        let board = dk::init().unwrap();

        let power = board.power;
        let counter: u64 = 0;

        power.intenset.write(|w| w.usbdetected().set_bit());

        defmt::println!("USBDETECTED interrupt enabled, counter is {}", counter);

        (
            MySharedResources {},
            MyLocalResources { power, counter: 0},
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            defmt::println!("idle: going to sleep");
            asm::wfi();
            defmt::println!("idle: woke up");
        }
    }

    #[task(binds = POWER_CLOCK, local = [power, counter])]
    //                                      ^^^^^^^ resource access list
    fn on_power_event(cx: on_power_event::Context) {
        defmt::println!("POWER event occurred (local resource) {}", cx.local.counter);

        // resources available to this task
        let resources = cx.local;

        // clear the interrupt flag; otherwise this task will run again after it returns
        resources.power.events_usbdetected.reset();

        let counter = resources.counter;
        *counter += 1;
    }
}
