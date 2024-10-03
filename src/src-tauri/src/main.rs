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
  let board = board(fen);
  format!("{}", board)
}

fn board(fen: &str) -> String {
    let fen = fen.split(' ').next().unwrap_or(fen);
    let mut position_string = String::new();
    for i in fen.chars() {
        position_string = format!("{}{}", position_string, filter(i));
    }
    return position_string;
}

fn filter(i: char) -> String {
    if i == '/' {
        return "".to_string();
    } 
    else if i.is_digit(10) {
        let numblank = i.to_digit(10).unwrap();
        return space(numblank);
    }
    return i.to_string();
}

fn space(i: u32) -> String{
    let mut spaces = String::new();
    for _x in 0..i {
        spaces.push(' ');
    }
    spaces
}