
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32_metapac as pac;
use pac::gpio::vals::{Moder, Idr};
use pac::gpio::vals::Pupdr;
#[entry]
fn main() -> ! {
    // Enable GPIOA clock
    pac::RCC.ahbenr().modify(|w| w.set_gpioaen(true));

    // Enable GPIOC clock
    pac::RCC.ahbenr().modify(|w| w.set_gpiocen(true));

    // Set PA5 as output
    pac::GPIOA.moder().modify(|w| w.set_moder(5, Moder::OUTPUT));

    // Set PC13 as input
    pac::GPIOC.moder().modify(|w| w.set_moder(13, Moder::INPUT));

    // enable pull up for PC13
    pac::GPIOC.pupdr().modify(|w| w.set_pupdr(13, Pupdr::PULL_UP));

    
    let mut led_on=false;
    let mut prev_button_state=Idr::HIGH;

    loop {
        let current_button_state= pac::GPIOC.idr().read().idr(13);
        if prev_button_state==Idr::HIGH && current_button_state==Idr::LOW{
            led_on=!led_on;
        }
        prev_button_state=current_button_state;

        if led_on{
            pac::GPIOA.bsrr().write(|w|w.set_bs(5, true));
        }else{
            pac::GPIOA.bsrr().write(|w| w.set_br(5, true));
        }
    }
}