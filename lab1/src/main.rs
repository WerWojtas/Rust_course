use std::io;

fn main() {
    println!("Welcome to tictactoe!");
    println!("Player 1 is X, Player 2 is O");
    let mut board = [[' '; 3]; 3];
    print_board(board);
    let mut player = 1;
    let mut sign: char;
    for _ in 0..9{
        let row : usize;
        let col : usize;
        let player_num : i32;
        if player % 2 == 0 {
            (row, col) = get_command(2);
            sign = 'O';
            player_num = 2;
        }
        else {
            (row, col) = get_command(1);
            sign = 'X';
            player_num = 1;
        }
        board[row][col] = sign;
        print_board(board);
        if check_win(board){
            println!("Player {:?} wins!", player_num);
            break;
        }
        player += 1;

    }
}

fn get_command(player_num: i32) -> (usize, usize) {
    let mut player = String::new();
    let row : char;
    let col : char;
    println!("Player {:?} type row and column numbers:", player_num);
    let _ = io::stdin().read_line(&mut player); // get string from the user input
    row = player.chars().nth(0).unwrap(); // get the first char from the given string
    col = player.chars().nth(2).unwrap(); // get the second char from the given string
    match (row.to_digit(10), col.to_digit(10)) {
        (Some(row_digit), Some(col_digit)) => {
            match (usize::try_from(row_digit), usize::try_from(col_digit)) {
                (Ok(row), Ok(col)) if check_numbers(row, col) => (row - 1, col - 1),
                _ => {
                    println!("Invalid input type, or invalid numbers range, please try again in format row(space)col");
                    get_command(player_num)
                }
            }
        }
        _ => {
            println!("Invalid input type, please try again in format row(space)col");
            get_command(player_num)
        }
    }
}

fn check_numbers(row: usize, col: usize) -> bool {
    if row > 3 || col > 3 {
        return false;
    }
    return true;
}

fn print_board(board: [[char; 3]; 3]) {
    for i in 0..3 {
        println!("{:?}",board[i]);
    }
}

fn check_win(board: [[char; 3]; 3]) -> bool{
    for i in 0..3{
        if check_three(board[i][0], board[i][1], board[i][2]){
            return true;
        }
        if check_three(board[0][i], board[1][i], board[2][i]){
            return true;
        }
    }
    if check_three(board[0][0], board[1][1], board[2][2]){
        return true;
    }
    if check_three(board[0][2], board[1][1], board[2][0]){
        return true;
    }
    return false;

}

fn check_three(first: char, second: char, third: char) -> bool {
    if first == ' ' || second == ' ' || third == ' ' {
        return false;
    }
    if first == second && second == third {
        return true;
    }
    return false;
}



