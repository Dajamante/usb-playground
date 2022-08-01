use dioxus::prelude::*;
use postcard::{from_bytes_cobs, to_slice_cobs};
use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::io;
use std::time::Duration;

#[derive(Debug, Serialize)]
enum Command {
    On,
    Off,
    Temperature,
}

#[derive(Debug, Deserialize)]
enum Response {
    Ack,
    Nack,
    Temperature(f32),
}
#[derive(Debug)]
pub enum USBError {
    BadCommand,
}
impl TryFrom<&str> for Command {
    type Error = USBError;

    fn try_from(s: &str) -> Result<Command, USBError> {
        match s {
            "on" => Ok(Command::On),
            "off" => Ok(Command::Off),
            "temp" => Ok(Command::Temperature),
            _ => {
                println!("Unknown command");
                Err(USBError::BadCommand)
            }
        }
    }
}

fn main() {
    dioxus::desktop::launch(app);
}

fn get_temp(buf: &mut [u8; 64], port: &mut Box<dyn SerialPort>) {
    // That returns the number of bytes
    let command = Command::Temperature;
    if let Ok(data) = to_slice_cobs(&command, buf) {
        port.write_all(data).unwrap();
    }
    if let Ok(count) = port.read(buf) {
        if let Ok(response) = from_bytes_cobs::<Response>(&mut buf[..count]) {
            println!("{:?}", response);
        }
    }
}

fn app(cx: Scope) -> Element {
    let mut port = init()?;
    let mut buf = [0; 64];

    cx.render(rsx! (
        div {
            background_color: "orange",
            h1    {"Interfacing sensor with USB."}
            p     {"Click on the buttons to have information from the board."}
        },
        button {
            onclick: move |_evt| get_temp(&mut buf,&mut port),
            "Temperature!"
        },

    ))
}
fn init() -> Option<Box<dyn SerialPort>> {
    let mut dport = None;
    for port in serialport::available_ports().unwrap() {
        if let serialport::SerialPortType::UsbPort(serialport::UsbPortInfo {
            serial_number: Some(sn),
            ..
        }) = &port.port_type
        {
            // Serial number must be same as in the firmware
            if sn.as_str() == "InOurEyes" {
                dport = Some(port.clone());
                break;
            }
        }
    }

    // let dport = if let Some(port) = dport {
    //     port
    // } else {
    //     return None;
    // };
    let dport = dport?;
    let mut port = serialport::new(dport.port_name, 115200)
        .timeout(Duration::from_millis(5))
        .open()
        .map_err(drop)
        .ok()?;
    println!("Possible instructions: \n on \n off \n temp");
    Some(port)
}
