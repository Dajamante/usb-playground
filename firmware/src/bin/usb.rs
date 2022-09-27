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
use postcard::{from_bytes_cobs, to_slice_cobs};
use serde::{Deserialize, Serialize};
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};
#[derive(Debug, defmt::Format, Deserialize)]
enum Command {
    On,
    Off,
    Temperature,
}

#[derive(Debug, Serialize)]
enum Response {
    Ack,
    Nack,
    Temperature(f32),
}

#[entry]
fn main() -> ! {
    let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();
    let port0 = nrf52840_hal::gpio::p0::Parts::new(periph.P0);
    let mut led = port0.p0_13.into_push_pull_output(Level::High).degrade();

    let mut temp_sensor = Temp::new(periph.TEMP);

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

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                if let Ok(command) = from_bytes_cobs(&mut buf) {
                    info!("received {}", command);
                    let mut response = Response::Nack;
                    match command {
                        Command::On => {
                            response = Response::Ack;
                            led.set_low();
                        }
                        Command::Off => {
                            response = Response::Ack;
                            led.set_high();
                        }
                        Command::Temperature => {
                            let temp: f32 = temp_sensor.measure().to_num();
                            defmt::info!("processor temp is {}°C", temp);
                            response = Response::Temperature(temp);
                        }
                    }
                    let data = to_slice_cobs(&response, &mut buf).unwrap();
                    serial.write(data).unwrap();
                }
            }
            _ => {}
        }
    }
}
