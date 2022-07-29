#![no_std]
#![no_main]

use usb as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use defmt::info;
use nrf52840_hal::{
    clocks::Clocks,
    gpio::Level,
    prelude::OutputPin,
    temp::Temp,
    usbd::{UsbPeripheral, Usbd},
};
use postcard::from_bytes_cobs;
use serde::Deserialize;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
#[derive(Debug, defmt::Format, Deserialize)]
enum Command {
    On,
    Off,
    Temperature,
}
#[entry]
fn main() -> ! {
    let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();
    let mut port0 = nrf52840_hal::gpio::p0::Parts::new(periph.P0);
    let mut led = port0.p0_13.into_push_pull_output(Level::High).degrade();

    let mut temp_sensor = Temp::new(periph.TEMP);

    let usb_bus = Usbd::new(UsbPeripheral::new(periph.USBD, &clocks));
    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("AllTheTears")
        .product("FromTheDust")
        .serial_number("InOurEyes")
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                if let Ok(command) = from_bytes_cobs(&mut buf) {
                    info!("received {}", command);
                    match command {
                        Command::On => led.set_low(),
                        Command::Off => led.set_high(),
                        Command::Temperature => {
                            let temp: i32 = temp_sensor.measure().to_num();
                            defmt::info!("processor temp is {}°C", temp);
                            Ok({})
                        }
                    };
                }
            }
            _ => {}
        }
    }
}
