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
  println!();

  let mut positions: Vec<char> = Vec::new();
  for i in fen.chars(){
      if i == ' '{
          break;
      }
      if i == '/'{
          continue;
      }
      else if i.is_digit(10)  {
          let numblank = i.to_digit(10).unwrap();
          for _i in 0..numblank{
              positions.push(' ');
          }
      }
      else {
          positions.push(i);
      }
  }
  positions.into_iter().collect()
}
