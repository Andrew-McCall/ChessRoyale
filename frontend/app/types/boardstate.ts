import { packSquare, unpackSquare, type BoardSquare } from "./boardsquare";

export class BoardState {
  width: number;
  height: number;
  pieces: Int16Array;

  constructor(width: number, height: number);
  constructor(width: number, height: number, buffer: Int16Array);
  constructor(width: number, height: number, buffer?: Int16Array) {
    this.width = width;
    this.height = height;
    this.pieces = buffer ?? new Int16Array(width * height);
  }

  getIndex(x: number, y: number): number {
    return y * this.width + x;
  }

  get(index: number): BoardSquare {
    return unpackSquare(this.pieces[index] ?? 0);
  }

  getByIndex(x: number, y: number) {
    return this.get(this.getIndex(x, y));
  }

  set(x: number, y: number, square: BoardSquare): void {
    this.pieces[this.getIndex(x, y)] = packSquare(square);
  }
}
