use std::time::{Duration, Instant};
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

    let mut buf = [0; 64];
    loop {
        if last_send.elapsed() >= Duration::from_secs(1) {
            let str = "hello!\n";
            print!("TX: {}", str);
            port.write_all(str.as_bytes()).unwrap();

            last_send = Instant::now();
        }

        // 99.9999% error
        if let Ok(count) = port.read(&mut buf) {
            println!("{:?}", core::str::from_utf8(&buf[..count]));
        }
    }
}
