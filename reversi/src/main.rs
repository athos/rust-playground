extern crate reversi;

use reversi::board::{Square,Pos,Board};

use std::io;
use std::io::Write;
use std::str::FromStr;

fn read_in_pos() -> Pos {
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
            board.print();
        }

        if !board.has_available_pos(turn) {
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

        let pos = read_in_pos();
        let poses = board.flippable_poses(&pos, turn);
        if !poses.is_empty() {
            board.put_at(&pos, turn);
            board.flip(turn, &poses);

            turn = next_turn(turn);
        } else {
            println!("You can't. Try again.");
        }
    }
}

fn main() {
    let ref mut board = Board::new(8);
    board.init();

    main_loop(board, Square::Black);
}
