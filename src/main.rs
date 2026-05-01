// #![no_std]
// #![no_main]
// use cortex_m::asm::nop;
// use cortex_m_rt::entry;
// use panic_halt as _;
// use stm32_metapac::stm32f3::stm32f303::Peripherals;
// // use rtt_target::{rprintln, rtt_init_print};
// #[entry]
// fn main()-> ! {
//     let mut x: usize=0;
//     // rtt_init_print!();
//     // rprintln!("Hello, world!");
//     loop {
//         // rprintln!("echo...");
//         x+=1;
//         for _ in 0..x{
//             nop();
//         }
//     }
// }

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm;
use panic_halt as _;
use stm32_metapac as pac;

use pac::gpio::vals::{Moder, Odr};

#[entry]
fn main() -> ! {
    // Enable GPIOA clock
    pac::RCC.ahbenr().modify(|w| w.set_gpioaen(true));
    // pac::RCC.ahbenr().modify(|w| w.set_gpiocen(true));
    // Set PA5 as output
    pac::GPIOA.moder().modify(|w| w.set_moder(5, Moder::OUTPUT));

    loop {
        // Read current state
        // let state = pac::GPIOA.odr().read().odr(5);

        // Toggle
        // let new_state = match state {
        //     Odr::LOW => Odr::HIGH,
        //     Odr::HIGH => Odr::LOW,
        // };

         pac::GPIOA.odr().modify(|w| w.set_odr(5, Odr::LOW));
         asm::delay(8_000_000);
         pac::GPIOA.odr().modify(|w| w.set_odr(5, Odr::HIGH));
         asm::delay(8_000_000);
        // delay();
    }
}

fn delay() {
    for _ in 0..8_000_000 {
        asm::nop();
    }
}