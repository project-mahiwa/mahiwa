// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board_info;
mod flash;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // パスの取得系のAPI→https://docs.rs/tauri/1.5.3/tauri/api/path/index.html
            let local_data_dir = app.path_resolver().app_local_data_dir().unwrap();
            let cache_dir = app.path_resolver().app_cache_dir().unwrap();
            let mahiwa_dir = local_data_dir.join("mahiwa");
            let backend_dir = mahiwa_dir.join("mahiwa-backend");

            // cache_dirにディレクトリなければ作る
            if !cache_dir.exists() {
                std::fs::create_dir_all(&cache_dir).unwrap();
            }
            // local_data_dirににディレクトリなければ作る
            if !mahiwa_dir.exists() {
                std::fs::create_dir_all(&mahiwa_dir).unwrap();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            flash::flash_to_mcu,
            flash::get_boards_for_select
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
