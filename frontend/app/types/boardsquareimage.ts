import { PieceColor, PieceType, type BoardSquare } from "./boardsquare";

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
