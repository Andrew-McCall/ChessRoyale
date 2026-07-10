pub enum Piece {
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,

    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
}

impl Piece {
    pub fn is_white(&self) -> bool {
        match self {
            Piece::BlackPawn => false,
            Piece::BlackRook => false,
            Piece::BlackKnight => false,
            Piece::BlackBishop => false,
            Piece::BlackQueen => false,
            Piece::BlackKing => false,

            Piece::WhitePawn => true,
            Piece::WhiteRook => true,
            Piece::WhiteKnight => true,
            Piece::WhiteBishop => true,
            Piece::WhiteQueen => true,
            Piece::WhiteKing => true,
        }
    }

    pub fn to_u16(&self) -> u16 {
        match self {
            Piece::BlackPawn => 1 + PAWN_VALUE,
            Piece::BlackRook => 1 + ROOK_VALUE,
            Piece::BlackKnight => 1 + KNIGHT_VALUE,
            Piece::BlackBishop => 1 + BISHOP_VALUE,
            Piece::BlackQueen => 1 + QUEEN_VALUE,
            Piece::BlackKing => 1 + KING_VALUE,

            Piece::WhitePawn => PAWN_VALUE,
            Piece::WhiteRook => ROOK_VALUE,
            Piece::WhiteKnight => KNIGHT_VALUE,
            Piece::WhiteBishop => BISHOP_VALUE,
            Piece::WhiteQueen => QUEEN_VALUE,
            Piece::WhiteKing => KING_VALUE,
        }
    }

    pub fn from_u16(value: u16) -> Self {
        let value = value & 0x00FF;
        if value & 1 == 0 {
            let value = value - 1;
            match value {
                PAWN_VALUE => Piece::BlackPawn,
                ROOK_VALUE => Piece::BlackRook,
                KNIGHT_VALUE => Piece::BlackKnight,
                BISHOP_VALUE => Piece::BlackBishop,
                QUEEN_VALUE => Piece::BlackQueen,
                KING_VALUE => Piece::BlackKing,
                _ => panic!("Invalid Value"),
            }
        } else {
            match value {
                PAWN_VALUE => Piece::WhitePawn,
                ROOK_VALUE => Piece::WhiteRook,
                KNIGHT_VALUE => Piece::WhiteKnight,
                BISHOP_VALUE => Piece::WhiteBishop,
                QUEEN_VALUE => Piece::WhiteQueen,
                KING_VALUE => Piece::WhiteKing,
                _ => panic!("Invalid Value"),
            }
        }
    }
}

const PAWN_VALUE: u16 = 2;
const ROOK_VALUE: u16 = 4;
const KNIGHT_VALUE: u16 = 8;
const BISHOP_VALUE: u16 = 16;
const QUEEN_VALUE: u16 = 32;
const KING_VALUE: u16 = 64;

pub fn is_u16_white(value: u16) -> bool {
    value & 1 == 0
}
