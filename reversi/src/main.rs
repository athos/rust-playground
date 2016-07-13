use std::io;
use std::io::Write;
use std::str::FromStr;

const BOARD_HEIGHT: isize = 8;
const BOARD_WIDTH: isize = 8;
const BOARD_SIZE: usize = (BOARD_HEIGHT * BOARD_HEIGHT) as usize;

#[derive(Copy,Clone,PartialEq)]
enum Square {
    Black,
    White,
    Empty
}

type Board = [Square; BOARD_SIZE];

fn get_at(board: &Board, y: isize, x: isize) -> Square {
    board[(BOARD_WIDTH * y + x) as usize]
}

fn put_at(board: &mut Board, y: isize, x: isize, s: Square) {
    board[(BOARD_WIDTH * y + x) as usize] = s;
}

fn print_board(board: &Board) {
    println!("  1 2 3 4 5 6 7 8");
    println!(" +-+-+-+-+-+-+-+-+");
    for y in 0..BOARD_HEIGHT {
        print!("{}|", y + 1);
        for x in 0..BOARD_WIDTH {
            match get_at(board, y, x) {
                Square::Black => print!("o"),
                Square::White => print!("x"),
                Square::Empty => print!(" ")
            }
            print!("|");
        }
        print!("\n");
    }
    println!(" +-+-+-+-+-+-+-+-+");
}

fn init_board(board: &mut Board) {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let dy = y - BOARD_HEIGHT/2 + 1;
            let dx = x - BOARD_WIDTH/2 + 1;
            let s = match (dy, dx) {
                (0, 0) => Square::Black,
                (0, 1) => Square::White,
                (1, 0) => Square::White,
                (1, 1) => Square::Black,
                (_, _) => Square::Empty
            };
            put_at(board, y, x, s);
        }
    }
}

fn safe_get_at(board: &Board, y: isize, x: isize) -> Option<Square> {
    if y < 0 || BOARD_HEIGHT <= y || x < 0 || BOARD_WIDTH <= x {
        None
    } else {
        Some(get_at(board, y, x))
    }
}

fn flippable_disks_for(board: &Board, y: isize, x: isize, dy: isize, dx: isize, s: Square) -> Option<Vec<(isize,isize)>> {
    let mut y_next = y + dy;
    let mut x_next = x + dx;
    let mut disks = Vec::new();

    while let Some(s_next) = safe_get_at(board, y_next, x_next) {
        if s_next == Square::Empty {
            break;
        } else if s_next == s {
            return if disks.len() == 0 { None } else { Some(disks) };
        }
        disks.push((y_next, x_next));
        y_next += dy;
        x_next += dx;
    }
    return None;
}

fn flippable_disks(board: &Board, y: isize, x: isize, s: Square) -> Vec<(isize,isize)> {
    let mut ret = Vec::new();

    for dy in -1..2 {
        for dx in -1..2 {
            if dy == 0 && dx == 0 { continue }

            if let Some(mut disks) = flippable_disks_for(board, y, x, dy, dx, s) {
                ret.append(&mut disks);
            }
        }
    }
    return ret;
}

fn flip(board: &mut Board, s: Square, disks: &[(isize,isize)]) {
    for &(y, x) in disks {
        put_at(board, y, x, s);
    }
}

fn has_available_pos(board: &Board, s: Square) -> bool {
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if get_at(board, y, x) == Square::Empty
            && !flippable_disks(board, y, x, s).is_empty() {
                return true;
            }
        }
    }
    return false;
}

fn read_in_pos() -> (isize, isize) {
    let mut line = String::new();
    io::stdin().read_line(&mut line);

    let inputs = &line.split(' ').collect::<Vec<_>>();
    let y = isize::from_str(inputs[0].trim()).unwrap() - 1;
    let x = isize::from_str(inputs[1].trim()).unwrap() - 1;

    return (y, x);
}

fn main_loop(board: &mut Board, turn: Square) {
    let mut turn = turn;
    let mut passed = false;

    let next_turn = |s| {
        if s == Square::Black { Square::White } else { Square::Black }
    };

    loop {
        if !passed {
            print!("\n");
            print_board(board);
        }

        if !has_available_pos(board, turn) {
            if passed {
                println!("Game over\n");
                return;
            } else {
                turn = next_turn(turn);
                passed = true;
                continue;
            }
        }

        passed = false;
        let s = if turn == Square::Black { "o" } else { "x" };
        print!("\n{}'s turn> ", s);
        io::stdout().flush();

        let (y, x) = read_in_pos();
        let disks = flippable_disks(board, y, x, turn);
        if !disks.is_empty() {
            put_at(board, y, x, turn);
            flip(board, turn, &disks);

            turn = next_turn(turn);
        } else {
            println!("You can't. Try again.");
        }
    }
}

fn main() {
    let ref mut board = [Square::Empty; BOARD_SIZE];
    init_board(board);

    main_loop(board, Square::Black);
}
