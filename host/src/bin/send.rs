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

// Bear with me 🐻.
#[derive(Debug)]
pub enum MyUSBError {
    BadCommand,
    // Error: Error { kind: Unknown, description: "Device or resource busy" }
    // that's a serialError https://github.com/serialport/serialport-rs/blob/6542d11235532ec78332e1e6b4986e73b8d55b11/src/lib.rs#L76
    // Meaning the board is used in coolterm for example!
    SerialDevideBusy(serialport::Error),
}
impl TryFrom<&str> for Command {
    type Error = MyUSBError;

    fn try_from(s: &str) -> Result<Command, MyUSBError> {
        match s {
            "on" => Ok(Command::On),
            "off" => Ok(Command::Off),
            "temp" => Ok(Command::Temperature),
            _ => {
                println!("Unknown command");
                Err(MyUSBError::BadCommand)
            }
        }
    }
}

fn main() -> Result<(), MyUSBError> {
    let mut dport = None;

    for port in serialport::available_ports().unwrap() {
        if let serialport::SerialPortType::UsbPort(serialport::UsbPortInfo {
            serial_number: Some(sn),
            ..
        }) = &port.port_type
        {
            println!("sn {}", &sn);
            // Serial number must be same as in the firmware
            if sn.as_str() == "warpigs" {
                dport = Some(port.clone());
                println!("assigned port.");
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

    println!("before port box dyn");
    let mut port = serialport::new(dport.port_name, 115200)
        .timeout(Duration::from_millis(5))
        .open()
        .map_err(|e| MyUSBError::SerialDevideBusy(e))?;
    println!("after port box dyn");

    let mut buf = [0; 64];
    // Nothing fancy here, just trimming the input
    println!("Possible instructions: \n on \n off \n temp");

    loop {
        let mut input = String::new();
        // `if let Ok(_)` is redundant pattern matching. Inside the `Ok()` we return the number of bytes
        if io::stdin().read_line(&mut input).is_ok() {
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
