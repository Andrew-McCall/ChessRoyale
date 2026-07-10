use crate::{
    action::Action::{self, Move, Take},
    piece::Piece,
};

const MAX: usize = 64; // 8x8 but will be: 30 * 10;

pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: [u16; MAX],
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_from_cells(width, height, [0; MAX])
    }

    pub fn new_from_cells(width: usize, height: usize, cells: [u16; MAX]) -> Self {
        Board {
            width,
            height,
            cells,
        }
    }

    pub fn new_from_board(&self) -> Self {
        Board {
            width: self.width,
            height: self.height,
            cells: self.cells,
        }
    }

    pub fn translate_index(&self, index: usize, dx: isize, dy: isize) -> Option<usize> {
        let (x, y) = self.xy_at(index);

        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x < 0 || new_y < 0 {
            return None;
        }

        let new_x = new_x as usize;
        if new_x >= self.width {
            return None;
        }

        let new_y = new_y as usize;
        if new_y >= self.height {
            return None;
        }

        Some(self.index_at(new_x, new_y))
    }

    pub fn get_cell(&self, index: usize) -> Option<Piece> {
        let piece_val: u16 = self.cells[index];
        if piece_val == 0 {
            return None;
        }
        Some(Piece::from_u16(piece_val))
    }

    pub fn set_cell(&mut self, index: usize, value: Option<Piece>) {
        self.cells[index] = value.map(|piece| piece.to_u16()).unwrap_or(0);
    }

    pub fn index_at(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn xy_at(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn calculate_legal_moves(&self, index: usize) -> Vec<Action> {
        let piece = match self.get_cell(index) {
            Some(piece) => piece,
            None => return Vec::new(),
        };
        let mut actions = Vec::new();

        match piece {
            Piece::BlackQueen | Piece::WhiteQueen => {
                let mut x = 0;
                let mut y = 0;
                for (dx, dy) in QUEEN_MOVES {
                    x += dx;
                    y += dy;
                    while let Some(new_index) = self.translate_index(index, x, y) {
                        match self.get_cell(new_index) {
                            Some(piece) => {
                                if piece.is_white() != piece.is_white() {
                                    actions.push(Take(new_index));
                                }
                                break;
                            }
                            None => {
                                actions.push(Move(new_index));
                            }
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
            Piece::BlackRook | Piece::WhiteRook => {
                let mut x = 0;
                let mut y = 0;
                for (dx, dy) in ROOK_MOVES {
                    x += dx;
                    y += dy;
                    while let Some(new_index) = self.translate_index(index, x, y) {
                        match self.get_cell(new_index) {
                            Some(piece) => {
                                if piece.is_white() != piece.is_white() {
                                    actions.push(Take(new_index));
                                }
                                break;
                            }
                            None => {
                                actions.push(Move(new_index));
                            }
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
            Piece::BlackBishop | Piece::WhiteBishop => {
                let mut x = 0;
                let mut y = 0;
                for (dx, dy) in BISHOP_MOVES {
                    x += dx;
                    y += dy;
                    while let Some(new_index) = self.translate_index(index, x, y) {
                        match self.get_cell(new_index) {
                            Some(piece) => {
                                if piece.is_white() != piece.is_white() {
                                    actions.push(Take(new_index));
                                }
                                break;
                            }
                            None => {
                                actions.push(Move(new_index));
                            }
                        }
                        x += dx;
                        y += dy;
                    }
                }
            }
            _ => todo!(),
        }

        actions
    }
}

const ROOK_MOVES: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const BISHOP_MOVES: [(isize, isize); 4] = [(1, 1), (1, -1), (-1, -1), (-1, -1)];
const QUEEN_MOVES: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, -1),
];
