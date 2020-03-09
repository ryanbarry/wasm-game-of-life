import { set_panic_hook, Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// enhance rust debugging
set_panic_hook();

const CELL_SIZE   = 8; // px
const GRID_COLOR  = "#CCCCCC";
const DEAD_COLOR  = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();
let animationId = null;

// canvas should include 1px border around all cells
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawCells();
});

const renderLoop = () => {
    universe.tick();

    //drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

const ctx = canvas.getContext('2d');

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // horizontal
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const bitIsSet = (n, arr) => {
    const byte = Math.floor(n/8);
    const mask = 1 << (n%8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width*height/8);

    ctx.beginPath();

    for(let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.fillStyle = bitIsSet(idx, cells) ? ALIVE_COLOR : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};


const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");
playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

const play = () => {
    playPauseButton.textContent = "\u23F8";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "\u25B6";
    cancelAnimationFrame(animationId);
    animationId = null;
};

const resetButton = document.getElementById("reset");
resetButton.addEventListener("click", event => {
    pause();
    universe.randomize_state();
    drawCells();
});

const clearButton = document.getElementById("clear");
clearButton.addEventListener("click", event => {
    pause();
    universe.clear();
    drawCells();
});

drawGrid();
drawCells();
drawGrid();
play();
