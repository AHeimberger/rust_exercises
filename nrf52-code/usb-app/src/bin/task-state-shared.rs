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
    }

    #[shared]
    struct MySharedResources {
        counter: u8
    }

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        let board = dk::init().unwrap();

        let power = board.power;
        let counter: u8 = 0;

        power.intenset.write(|w| w.usbdetected().set_bit());

        defmt::println!("USBDETECTED interrupt enabled");

        (
            MySharedResources { counter },
            MyLocalResources { power },
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

    #[task(binds = POWER_CLOCK, local = [power], shared = [counter])]
    //                                      ^^^^^^^ resource access list
    fn on_power_event(mut cx: on_power_event::Context) {
        let counter = cx.shared.counter.lock(| counter | {
            *counter += 1;
            *counter
        });
        defmt::println!("POWER event occurred (shared resource) {}", counter);

        // resources available to this task
        let resources = cx.local;

        // clear the interrupt flag; otherwise this task will run again after it returns
        resources.power.events_usbdetected.reset();
    }
}
