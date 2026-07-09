const COLOR_SHIFT = 0;
const COLOR_BITS = 1;
const PIECE_SHIFT = COLOR_SHIFT + COLOR_BITS;
const PIECE_BITS = 5;

const COLOR_MASK = (1 << COLOR_BITS) - 1;
const PIECE_MASK = (1 << PIECE_BITS) - 1;

export enum PieceColor {
  White = 0,
  Black = 1,
}

export enum PieceType {
  Pawn = 1,
  Rook = 2,
  Knight = 3,
  Bishop = 4,
  Queen = 5,
  King = 6,
}

export interface BoardSquare {
  color: PieceColor;
  piece: PieceType;
}

export function packSquare(square: BoardSquare): number {
  return (
    ((square.piece & PIECE_MASK) << PIECE_SHIFT) |
    ((square.color & COLOR_MASK) << COLOR_SHIFT)
  );
}

export function unpackSquare(value: number): BoardSquare {
  return {
    color: (value >> COLOR_SHIFT) & COLOR_MASK,
    piece: (value >> PIECE_SHIFT) & PIECE_MASK,
  };
}

const PIECE_IMAGES = import.meta.glob<string>("../assets/pieces-svg/*.svg", {
  eager: true,
  import: "default",
});

const PIECE_NAMES: Record<PieceType, string> = {
  [PieceType.Pawn]: "pawn",
  [PieceType.Rook]: "rook",
  [PieceType.Knight]: "knight",
  [PieceType.Bishop]: "bishop",
  [PieceType.Queen]: "queen",
  [PieceType.King]: "king",
};

const COLOR_SUFFIXES: Record<PieceColor, string> = {
  [PieceColor.White]: "w",
  [PieceColor.Black]: "b",
};

export function getBoardSquareImage(square: BoardSquare): string | undefined {
  const name = PIECE_NAMES[square.piece];
  if (!name) return undefined;

  const path = `../assets/pieces-svg/${name}-${COLOR_SUFFIXES[square.color]}.svg`;
  return PIECE_IMAGES[path];
}
