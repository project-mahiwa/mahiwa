use std::{io, thread, time};
use tauri::Window;

#[tauri::command]
pub fn serial(window: Window, port_name: &str, baud_rate: u32) -> Result<(), ()> {
    let mut serial_port = match serialport::new(port_name, baud_rate).open() {
        Ok(port) => port,
        Err(_) => return Err(()),
    };

    let mut serial_buf: Vec<u8> = vec![0; 1000];
    let read_timeout = time::Duration::from_millis(1000);

    // シリアル通信の結果をプロセス間通信でSvelteに横流しする
    thread::spawn(move || loop {
        match serial_port.read(serial_buf.as_mut_slice()) {
            Ok(t) => {
                let read_result = String::from_utf8_lossy(&serial_buf[..t]);
                window.emit("btf-flash-communication", read_result).unwrap();
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e),
        }
        thread::sleep(read_timeout);
    });
    return Ok(());
}

//利用可能なシリアルポートの一覧を取得する
#[tauri::command]
pub fn list_serial_ports() -> Vec<String> {
    let ports = serialport::available_ports().unwrap();
    let mut port_names: Vec<String> = Vec::new();
    for port in ports {
        port_names.push(port.port_name);
    }
    return port_names;
}
