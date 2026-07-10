use crate::piece::Piece;

pub enum Action {
    Move(usize),
    Take(usize),
    Promote(usize, Piece),
    Enpasant(usize, usize),
}
