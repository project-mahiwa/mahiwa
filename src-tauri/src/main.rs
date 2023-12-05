// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board_info;
mod flash;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            flash::flash_to_mcu,
            flash::get_boards_for_select
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
