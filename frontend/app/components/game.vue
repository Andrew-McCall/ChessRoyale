<script setup lang="ts">
import { ref } from 'vue'
import { BoardState } from '~/types/boardstate'
import { PieceColor, PieceType, type BoardSquare } from '~/types/boardsquare'
import { getBoardSquareImage } from '~/types/boardsquareimage'
import { DefaultTheme, type Theme } from '~/types/theme'

const { board = new BoardState(8, 8), theme = DefaultTheme } = defineProps<{
    board?: BoardState
    theme?: Theme
}>()

const version = ref(0)

const getCellPosition = (index: number) => ({
    row: Math.floor((index - 1) / board.width),
    col: (index - 1) % board.width,
})

const getCellColour = (index: number) => {
    const { row, col } = getCellPosition(index)

    return (row + col) % 2 === 0
        ? theme.white :
        theme.black
}

const getCellImage = (index: number) => {
    void version.value

    return getBoardSquareImage(board.get(index - 1))
}

const onCellClick = (index: number) => {
    const { row, col } = getCellPosition(index)
    const square = board.get(index - 1)

    const next: BoardSquare =
        square.piece !== PieceType.Pawn
            ? { piece: PieceType.Pawn, color: PieceColor.White }
            : square.color === PieceColor.White
                ? { piece: PieceType.Pawn, color: PieceColor.Black }
                : { piece: 0 as PieceType, color: PieceColor.White }

    board.set(col, row, next)
    version.value++
}
</script>

<template>
    <div class="grid w-full" :style="{
        gridTemplateColumns: `repeat(${board.width}, 1fr)`,
        aspectRatio: `${board.width} / ${board.height}`,
    }">
        <div v-for="cell in board.height * board.width" :key="cell"
            :class="[getCellColour(cell), 'relative cursor-pointer select-none']" @click="onCellClick(cell)">
            <img v-if="getCellImage(cell)" :src="getCellImage(cell)" class="absolute inset-0 w-full h-full" />
        </div>
    </div>
</template>