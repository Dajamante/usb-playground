use postcard::{from_bytes_cobs, to_slice_cobs};
use serde::{Deserialize, Serialize};
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

fn main() -> Result<(), ()> {
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

    let dport = if let Some(port) = dport {
        port
    } else {
        eprintln!("Error: No USB connected!");
        return Ok(());
    };

    let mut port = serialport::new(dport.port_name, 115200)
        .timeout(Duration::from_millis(5))
        .open()
        .map_err(drop)?;

    let mut buf = [0; 64];
    println!("Possible instructions: \n on \n off \n temp");

    loop {
        let mut input = String::new();
        // That returns the number of bytes
        if let Ok(_) = io::stdin().read_line(&mut input) {
            match Command::try_from(input.trim()) {
                Ok(command) => {
                    println!("Command::{:?}", command);
                    if let Ok(data) = to_slice_cobs(&command, &mut buf) {
                        port.write_all(data).unwrap();
                    }
                }
                Err(e) => eprintln!("Command::{:?}", e),
            }
        }

        if let Ok(count) = port.read(&mut buf) {
            if let Ok(response) = from_bytes_cobs::<Response>(&mut buf[..count]) {
                println!("{:?}", response);
            }
        }
    }
}
