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
