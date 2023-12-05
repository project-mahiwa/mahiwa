use crate::board_info;

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
pub fn flash_to_mcu(board_name: &str, wasm_file_path: &str) -> String {
    println!("board_name: {}", board_name);
    println!("wasm_file_path: {}", wasm_file_path);
    return "flash_to_mcu".to_string();
}
