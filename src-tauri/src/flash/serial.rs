use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::Window;
#[tauri::command]
pub fn serial(window: Window) -> Result<(), ()> {
    /*
     * pio monitorで見る
     */
    let mut child = Command::new("pio")
        .args(["device", "monitor", "-b", "115200"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to pio device monitor");
    let stdout = child.stdout.take().unwrap();
    // 1行ずつ結果を取る
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        window.emit("btf-flash-prgoress", line.unwrap()).unwrap();
    }
    return Ok(());
}
