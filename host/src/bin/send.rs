use std::time::{Duration, Instant};

use postcard::to_slice_cobs;
use serde::Serialize;

#[derive(Debug, Serialize)]
enum Command {
    On,
    Off,
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

    let mut last_send = Instant::now();
    let mut cmd = Command::On;
    let mut is_on = false;

    let mut buf = [0; 64];
    loop {
        if last_send.elapsed() >= Duration::from_secs(1) {
            if is_on {
                cmd = Command::Off;
                is_on = false;
            } else {
                cmd = Command::On;
                is_on = true;
            }
            let data = to_slice_cobs(&cmd, &mut buf).unwrap();

            port.write_all(data).unwrap();

            last_send = Instant::now();
        }

        // 99.9999% error
        if let Ok(count) = port.read(&mut buf) {
            println!("{:?}", core::str::from_utf8(&buf[..count]));
        }
    }
}
