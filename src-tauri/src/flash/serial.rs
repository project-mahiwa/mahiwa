use serialport::prelude::*;
use std::io::{self, Read};
use std::time::Duration;
use tauri::Window;

#[tauri::command]
pub fn serial(window: Window) -> Result<(), ()> {
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 9600;

    let settings = SerialPortSettings {
        baud_rate,
        timeout: Duration::from_secs(1),
        ..Default::default()
    };

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Reading data from {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
    // for line in reader.lines() {
    //     window.emit("btf-flash-prgoress", line.unwrap()).unwrap();
    // }
    return Ok(());
}
