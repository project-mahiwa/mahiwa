use crate::board_info;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use tauri::AppHandle;
use tauri::Window;

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
    app_handle: AppHandle,
    board_name: &str,
    wasm_file_path: &str,
) -> Result<String, String> {
    // /home/usuyuki/.local/share/net.usuyuki.mahiwa 的なのが取れる
    let local_data_dir = app_handle.path_resolver().app_local_data_dir().unwrap();
    // /home/usuyuki/.cache/net.usuyuki.mahiwa 的なのが取れる
    let cache_dir = app_handle.path_resolver().app_cache_dir().unwrap();
    let mahiwa_dir = local_data_dir.join("mahiwa");
    let backend_dir = mahiwa_dir.join("mahiwa-backend");

    /*
     * mahiwa-backendのgit clone(setupでなくここでやることで起動速度を防ぎ、かつ最新を使うようにする)
     */
    // mahiwa-backendディレクトリの存在確認
    if backend_dir.exists() {
        window.emit("btf-flash-prgoress", "git clone ...").unwrap();
        // Gitリポジトリが存在するか確認
        if backend_dir.join(".git").exists() {
            // Gitリポジトリを更新 fetch origin mainにして reset --hard origin/main方式もあり
            let output = Command::new("git")
                .args(["pull", "origin", "main"])
                .current_dir(&backend_dir)
                .output()
                .expect("Failed to pull git repository");
            window
                .emit(
                    "btf-flash-prgoress",
                    std::str::from_utf8(&output.stdout).unwrap(),
                )
                .unwrap();
        } else {
            // Gitリポジトリをクローン
            let output = Command::new("git")
                // @todo ここ公開したらssh外す
                .args([
                    "clone",
                    "git@github.com:project-mahiwa/mahiwa-backend.git",
                    &backend_dir.to_string_lossy(),
                ])
                .output()
                .expect("Failed to clone git repository");
            window
                .emit(
                    "btf-flash-prgoress",
                    std::str::from_utf8(&output.stdout).unwrap(),
                )
                .unwrap();
        }
    } else {
        // mahiwa-backendディレクトリとGitリポジトリを作成
        std::fs::create_dir_all(&backend_dir).unwrap();
        let output = Command::new("git")
            // @todo ここ公開したらssh外す
            .args([
                "clone",
                "git@github.com:project-mahiwa/mahiwa-backend.git",
                &backend_dir.to_string_lossy(),
            ])
            .output()
            .expect("Failed to clone git repository");
        window
            .emit(
                "btf-flash-prgoress",
                std::str::from_utf8(&output.stdout).unwrap(),
            )
            .unwrap();
    }

    /*
     * いただいたwasmをcpして一時ディレクトリに名前を変えて入れる
     * xxdする上でヘッダファイルの変数名はファイル名依存、xxdコマンドにはそういう機能がないので、cache_dirにuser.wasmファイルとしてコピーする
     */
    let tmp_wasm_path = cache_dir.join("user.wasm");
    let tmp_wasm_path_str = tmp_wasm_path.to_str().unwrap();
    Command::new("cp")
        .args(["-f", wasm_file_path, tmp_wasm_path_str])
        .output()
        .expect("Failed to cp");
    window
        .emit(
            "btf-flash-prgoress",
            format!("cp {} {}", wasm_file_path, tmp_wasm_path_str),
        )
        .unwrap();

    /*
     * xxdでヘッダファイルを作成
     */
    let flash_wasm_path = backend_dir.join("src/wasm/user.h");
    let flash_wasm_path_str = flash_wasm_path.to_str().unwrap();
    // xxdでリダイレクトがCommandでできないので一旦保持する
    // wasmファイル名がそのまま変数になるのでカレントディレクトリをtmpにする
    let output = Command::new("xxd")
        .args(&["-i", "user.wasm"])
        .current_dir(&cache_dir)
        .output()
        .expect("Failed to execute xxd");

    // xxdの出力をファイルに書き込む
    let mut file = File::create(flash_wasm_path_str).expect("Failed to create file");
    file.write_all(&output.stdout)
        .expect("Failed to write to file");
    window
        .emit(
            "btf-flash-prgoress",
            format!("xxd -i {} > {}", tmp_wasm_path_str, flash_wasm_path_str),
        )
        .unwrap();

    /*
     * get_boardsでpio_envrionment_nameを取得
     */
    let boards = board_info::get_boards();
    let board = boards.get(board_name).unwrap();
    let pio_envrionment_name = board.get_pio_envrionment_name();
    /*
     * pioで書き込み
     */
    let mut command = Command::new("pio")
        .args(["run", "-t", "upload", "-e", pio_envrionment_name])
        .current_dir(&backend_dir)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to pio run");

    if let Some(output) = command.stdout.take() {
        let reader = BufReader::new(output);

        // 標準出力を逐次読み取り
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    // Tauriのイベントとしてフロントエンドに送信
                    window
                        .emit("command-output", line)
                        .expect("イベントの送信に失敗しました");
                }
                Err(e) => eprintln!("エラー: {}", e),
            }
        }
    }
    return Ok("run".to_string());
}
