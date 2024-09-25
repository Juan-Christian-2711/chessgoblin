fn main() {
    board()
}

fn board() {
    let mut board = String::new();
    board.push_str("    <table class=\"chess-board\">
        <tbody>
            <tr>");
    for i in 1..8 {
        board.push_str("
                <th></th>");
    }
    println!("{}", board);
}
