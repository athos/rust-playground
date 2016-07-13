#[derive(Copy,Clone,PartialEq)]
pub enum Square {
    Black,
    White,
    Empty
}

pub struct Board {
    board: Vec<Square>,
    size: isize
}

pub fn new(size: usize) -> Board {
    Board {
        board: vec![Square::Empty; size * size],
        size: size as isize
    }
}

pub fn get_at(board: &Board, y: isize, x: isize) -> Square {
    board.board[(board.size * y + x) as usize]
}

pub fn put_at(board: &mut Board, y: isize, x: isize, s: Square) {
    board.board[(board.size * y + x) as usize] = s;
}

pub fn print_board(board: &Board) {
    println!("  1 2 3 4 5 6 7 8");
    println!(" +-+-+-+-+-+-+-+-+");
    for y in 0..board.size {
        print!("{}|", y + 1);
        for x in 0..board.size {
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

pub fn init_board(board: &mut Board) {
    for y in 0..board.size {
        for x in 0..board.size {
            let dy = y - board.size/2 + 1;
            let dx = x - board.size/2 + 1;
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
    if y < 0 || board.size <= y || x < 0 || board.size <= x {
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

pub fn flippable_disks(board: &Board, y: isize, x: isize, s: Square) -> Vec<(isize,isize)> {
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

pub fn flip(board: &mut Board, s: Square, disks: &[(isize,isize)]) {
    for &(y, x) in disks {
        put_at(board, y, x, s);
    }
}

pub fn has_available_pos(board: &Board, s: Square) -> bool {
    for y in 0..board.size {
        for x in 0..board.size {
            if get_at(board, y, x) == Square::Empty
            && !flippable_disks(board, y, x, s).is_empty() {
                return true;
            }
        }
    }
    return false;
}
