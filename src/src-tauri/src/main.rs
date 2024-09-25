// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![render])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn render(fen: &str) -> String {
  let mut board = String::new();
  board.push_str("<table class=\"chess-board\">");
  format!("{}", board)
}