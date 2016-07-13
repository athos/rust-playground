extern crate reversi;

use reversi::board;
use reversi::board::Square;
use reversi::board::Board;

use std::io;
use std::io::Write;
use std::str::FromStr;

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
            board::print_board(board);
        }

        if !board::has_available_pos(board, turn) {
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
        let disks = board::flippable_disks(board, y, x, turn);
        if !disks.is_empty() {
            board::put_at(board, y, x, turn);
            board::flip(board, turn, &disks);

            turn = next_turn(turn);
        } else {
            println!("You can't. Try again.");
        }
    }
}

fn main() {
    let ref mut board = board::new(8);
    board::init_board(board);

    main_loop(board, Square::Black);
}
