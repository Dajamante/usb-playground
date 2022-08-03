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
pub struct Board {
    port: Box<dyn SerialPort>,
}

impl Board {
    fn new() -> Option<Self> {
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
        Some(Board { port })
    }

    fn toggle_light(&mut self, command: Command) -> Result<(), ()> {
        let mut buf = [0; 64];

        if let Ok(data) = to_slice_cobs(&command, &mut buf) {
            self.port.write_all(data).unwrap();
        }

        if let Ok(count) = self.port.read(&mut buf) {
            if let Ok(response) = from_bytes_cobs::<Response>(&mut buf[..count]) {
                match response {
                    Response::Ack => Ok(()),
                    _ => Err(()),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
    fn get_temp(&mut self) -> Result<f32, ()> {
        let mut buf = [0; 64];

        // That returns the number of bytes
        let command = Command::Temperature;
        if let Ok(data) = to_slice_cobs(&command, &mut buf) {
            self.port.write_all(data).unwrap();
        }

        let temp = self
            .port
            .read(&mut buf)
            .map_err(drop)
            .and_then(|count| from_bytes_cobs::<Response>(&mut buf[..count]).map_err(drop))
            .and_then(|r| match r {
                Response::Temperature(t) => Ok(t),
                _ => Err(()),
            });
        // .map(|r| match r {
        //     Response::Temperature(t) => Ok(t),
        //     _ => Err(()),
        // })
        //.unwrap()
        //.map_err(drop);

        // if let Ok(count) = self.port.read(&mut buf) {
        //     if let Ok(response) = from_bytes_cobs::<Response>(&mut buf[..count]) {
        //         match response {
        //             Response::Temperature(t) => Ok(t),
        //             _ => Err(()),
        //         }
        //     } else {
        //         Err(())
        //     }
        // } else {
        //     Err(())
        // }
        temp
    }
}

fn main() {
    dioxus::desktop::launch(app);
}

// App körs varje gång den renderas om
fn app(cx: Scope) -> Element {
    // ::new är en FnOnce
    // let mut board = use_ref(&cx, Board::new);
    let board = use_ref(&cx, || Board::new().unwrap());
    // smart pointer Rc<T>
    //let mut temp = **use_state(&cx, || board.write().get_temp().unwrap());

    let temp = use_state(&cx, || board.write().get_temp().unwrap());
    let is_on = use_state(&cx, || false);
    cx.render(rsx! (
        div {
            background_color: "orange",
            h1    {"Interfacing sensor with USB."}
            p     {"Click on the buttons to have information from the board."}
        },
        button {
            onclick: move |_| { temp.set(board.write().get_temp().unwrap()); },
            "Temperature!"
        },
        button {
            onclick: move |_| {
                    if **is_on {
                        is_on.set(false);
                        board.write().toggle_light(Command::Off).unwrap();
                    } else {
                        is_on.set(true);
                        board.write().toggle_light(Command::On).unwrap();
                    }

                },
            "Toggle light."
        },
        div {
            p  { "Temperature: {temp}" }
        }
    ))
}
