//! examples/generics.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;
use rtic::Mutex;

#[rtic::app(device = lm3s6965)]
mod app {
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[resources]
    struct Resources {
        #[init(0)]
        shared: u32,
    }

    #[init]
    fn init(_: init::Context) -> (init::LateResources, init::Monotonics) {
        rtic::pend(Interrupt::UART0);
        rtic::pend(Interrupt::UART1);

        (init::LateResources {}, init::Monotonics())
    }

    #[task(binds = UART0, resources = [shared])]
    fn uart0(c: uart0::Context) {
        let mut state = 0;
        // second argument has type `resources::shared`
        super::advance(&mut state, c.resources.shared);
        hprintln!("UART0, state {}", state).unwrap();

        rtic::pend(Interrupt::UART1);

        debug::exit(debug::EXIT_SUCCESS);
    }

    #[task(binds = UART1, priority = 2, resources = [shared])]
    fn uart1(c: uart1::Context) {
        let mut state = 1;
        // second argument has type `resources::shared`
        super::advance(&mut state, c.resources.shared);

        hprintln!("UART1, state {}", state).unwrap();
    }
}

// the second parameter is generic: it can be any type that implements the `Mutex` trait
fn advance(state: &mut u32, mut shared: impl Mutex<T = u32>) {
    *state += 1;

    let (old, new) = shared.lock(|shared: &mut u32| {
        let old = *shared;
        *shared += *state;
        (old, *shared)
    });

    hprintln!("shared: {} -> {}", old, new).unwrap();
}
