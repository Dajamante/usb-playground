#![no_main]
#![no_std]

use usb as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    usb::exit()
}
