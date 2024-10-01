fn main() {
    let board = board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", board);
}

fn board(fen: &str) -> String {
    println!();

    let mut positions: Vec<char> = Vec::new();
    let fen = fen.split(' ').next().unwrap_or(fen);
    let mut position_string = String::new();
    for i in fen.chars() {
        positions_string.push(filter(i));
    }
    return position_string;
}

fn filter(i: char) -> String {
    if i == '/' {
        return "".to_string();
    } 
    else if i.is_digit(10) {
        let mut spaces = String::new();
        let numblank = i.to_digit(10).unwrap();
        for _i in 0..numblank {
            spaces.push(i);
        }
        return spaces;
    }
    return i.to_string();
}
