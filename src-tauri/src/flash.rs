use crate::board_info;
use tauri::{Manager, Window};

#[tauri::command]
pub fn get_boards_for_select() -> Vec<String> {
    let boards = board_info::get_boards();
    //boardsからnameだけを取り出した配列を作る
    let mut board_names = Vec::new();
    for (name, _) in boards.iter() {
        board_names.push(name.to_string());
    }
    return board_names;
}

#[tauri::command]
pub async fn flash_to_mcu(
    window: Window,
    board_name: &str,
    wasm_file_path: &str,
) -> Result<String, String> {
    println!("board_name: {}", board_name);
    println!("wasm_file_path: {}", wasm_file_path);
    std::thread::sleep(std::time::Duration::from_secs(4));
    //git clone
    window
        .emit("btf-flash-prgoress", "git clone completed")
        .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    // xxdでヘッダファイルに変換
    // btf-flash-prgoressへのログをエミット
    window
        .emit("btf-flash-prgoress", "xxd conversion completed")
        .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    // pioで書き込み
    window
        .emit("btf-flash-prgoress", "pio write completed")
        .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    // シリアル通信
    // btf-flash-prgoressへのログをエミット
    window
        .emit("btf-flash-prgoress", "serial communication completed")
        .unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    return Ok("run".to_string());
}
