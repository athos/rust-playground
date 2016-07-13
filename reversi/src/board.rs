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

impl Board {
    pub fn new(size: usize) -> Board {
        let mut board = Board {
            board: vec![Square::Empty; size * size],
            size: size as isize
        };
        &mut board.init();

        return board;
    }

    pub fn get_at(&self, y: isize, x: isize) -> Square {
        self.board[(self.size * y + x) as usize]
    }

    pub fn put_at(&mut self, y: isize, x: isize, s: Square) {
        self.board[(self.size * y + x) as usize] = s;
    }

    pub fn init(&mut self) {
        for y in 0..self.size {
            for x in 0..self.size {
                let dy = y - self.size/2 + 1;
                let dx = x - self.size/2 + 1;
                let s = match (dy, dx) {
                    (0, 0) => Square::Black,
                    (0, 1) => Square::White,
                    (1, 0) => Square::White,
                    (1, 1) => Square::Black,
                    (_, _) => Square::Empty
                };
                self.put_at(y, x, s);
            }
        }
    }

    pub fn print(&self) {
        println!("  1 2 3 4 5 6 7 8");
        println!(" +-+-+-+-+-+-+-+-+");
        for y in 0..self.size {
            print!("{}|", y + 1);
            for x in 0..self.size {
                match self.get_at(y, x) {
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

    fn safe_get_at(&self, y: isize, x: isize) -> Option<Square> {
        if y < 0 || self.size <= y || x < 0 || self.size <= x {
            None
        } else {
            Some(self.get_at(y, x))
        }
    }

    fn flippable_disks_for(&self, y: isize, x: isize, dy: isize, dx: isize, s: Square) -> Option<Vec<(isize,isize)>> {
        let mut y_next = y + dy;
        let mut x_next = x + dx;
        let mut disks = Vec::new();

        while let Some(s_next) = self.safe_get_at(y_next, x_next) {
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

    pub fn flippable_disks(&self, y: isize, x: isize, s: Square) -> Vec<(isize,isize)> {
        let mut ret = Vec::new();

        for dy in -1..2 {
            for dx in -1..2 {
                if dy == 0 && dx == 0 { continue }

                if let Some(mut disks) = self.flippable_disks_for(y, x, dy, dx, s) {
                    ret.append(&mut disks);
                }
            }
        }
        return ret;
    }

    pub fn flip(&mut self, s: Square, disks: &[(isize,isize)]) {
        for &(y, x) in disks {
            self.put_at(y, x, s);
        }
    }

    pub fn has_available_pos(&self, s: Square) -> bool {
        for y in 0..self.size {
            for x in 0..self.size {
                if self.get_at(y, x) == Square::Empty
                    && !self.flippable_disks(y, x, s).is_empty() {
                        return true;
                    }
            }
        }
        return false;
    }
}
