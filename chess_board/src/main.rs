fn main() {
    let board = board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("\n{}", board);
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