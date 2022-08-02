use dioxus::prelude::*;
use postcard::{from_bytes_cobs, to_slice_cobs};
use serde::{Deserialize, Serialize};
use serialport::SerialPort;
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

fn main() {
    dioxus::desktop::launch(app);
}

fn get_temp(port: &mut Box<dyn SerialPort>) -> Result<f32, ()> {
    let mut buf = [0; 64];

    // That returns the number of bytes
    let command = Command::Temperature;
    if let Ok(data) = to_slice_cobs(&command, &mut buf) {
        port.write_all(data).unwrap();
    }

    if let Ok(count) = port.read(&mut buf) {
        if let Ok(response) = from_bytes_cobs::<Response>(&mut buf[..count]) {
            match response {
                Response::Temperature(t) => Ok(t),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn app(cx: Scope) -> Element {
    let mut port = init()?;
    // smart pointer Rc<T>
    let mut temp = use_state(&cx, || get_temp(&mut port).unwrap());
    cx.render(rsx! (
        div {
            background_color: "orange",
            h1    {"Interfacing sensor with USB."}
            p     {"Click on the buttons to have information from the board."}
        },
        button {
            onclick: move |_| temp.modify(|_| get_temp(&mut port).unwrap()),
            "Temperature!"
        },
        div {
            p  { "Temperature: {temp}" }
        }

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

    let dport = dport?;
    let port = serialport::new(dport.port_name, 115200)
        .timeout(Duration::from_millis(5))
        .open()
        .map_err(drop)
        .ok()?;
    println!("Possible instructions: \n on \n off \n temp");
    Some(port)
}
