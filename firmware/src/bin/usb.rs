#![no_std]
#![no_main]

use usb as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use defmt::{info, Format};
use nrf52840_hal::clocks::Clocks;
use nrf52840_hal::usbd::{UsbPeripheral, Usbd};
use postcard::from_bytes_cobs;
use serde::Deserialize;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[derive(Debug, Deserialize, Format)]
enum Command {
    On,
    Off,
}

#[entry]
fn main() -> ! {
    let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

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

        let mut buf = [0u8; 32];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                if let Ok(command) = from_bytes_cobs(&mut buf) {
                    if let Command::On | Command::Off = command {
                        info!("received {}", command);
                    }
                }
            }
            _ => {}
        }
    }
}
