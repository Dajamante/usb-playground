#![no_std]
#![no_main]

use core::fmt::write;

use usb as _; // global logger + panicking-behavior + memory layout

use cortex_m_rt::entry;
use defmt::info;
use nrf52840_hal::{
    clocks::Clocks,
    usbd::{UsbPeripheral, Usbd},
};
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let usb_bus = Usbd::new(UsbPeripheral::new(periph.USBD, &clocks));
    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        // fun fact:
        // writing `ls -l /dev/serial/by-id` will give the tty port and the identification
        // usb-AllTheTears_FromTheDust_InOurEyes-if00 -> ../../ttyACM1
        .manufacturer("black")
        .product("sabbath")
        .serial_number("warpigs")
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();
    let mut buf_read = [0_u8; 16];
    // Early return if there is no data to manage.
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        match serial.read(&mut buf_read) {
            // Ok(#number of read bytes)
            Ok(count) if count > 0 => {
                for c in buf_read[..count].iter_mut() {
                    if let Ok(c) = core::str::from_utf8(&[*c]) {
                        info!("{}", c);
                    }

                    if 0x61 <= *c && *c <= 0x7a {
                        //fancy way to do *c -= 32
                        *c &= !0x20;
                        let _ = serial.write(&[*c]);
                    }
                }
            }
            Err(e) => defmt::error!("USB error: {:?}", e),
            Ok(_) => {}
        }
    }
}
