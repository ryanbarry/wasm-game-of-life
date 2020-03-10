import {
    set_panic_hook,
    Universe,
    CellCoord as C
} from "wasm-game-of-life";
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

const clientXYtoColRow = (clientX, clientY) => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (clientX - boundingRect.left) * scaleX;
    const canvasTop = (clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    return { col, row };
};

const toggleCellAtClientXY = (clientX, clientY) => {
    let { col, row } = clientXYtoColRow(clientX, clientY);
    universe.toggle_cell(new C(col, row));
};

const translateCoordListByXY = (l, x, y) => {
    return l.map((xy, i) => {if (i%2===0) {return xy+x;} else {return xy+y;}});
};

const makeSpaceShipAtClientXY = (clientX, clientY) => {
    let { col, row } = clientXYtoColRow(clientX, clientY);

    let ss = [0, 1, 1, 2, 2, 0, 2, 1, 2, 2];
    ss = translateCoordListByXY(ss, col, row);
    universe.set_cells_by_coords(ss);
};

const makePulsarAtClientXY = (clientX, clientY) => {
    let { col, row } = clientXYtoColRow(clientX, clientY);

    let pulsar = [3, 1, 4, 1, 5, 1, 9, 1, 10, 1, 11, 1,
                  1, 3, 6, 3, 8, 3, 13, 3,
                  1, 4, 6, 4, 8, 4, 13, 4,
                  1, 5, 6, 5, 8, 5, 13, 5,
                  3, 6, 4, 6, 5, 6, 9, 6, 10, 6, 11, 6,
                  3, 8, 4, 8, 5, 8, 9, 8, 10, 8, 11, 8,
                  1, 9, 6, 9, 8, 9, 13, 9,
                  1, 10, 6, 10, 8, 10, 13, 10,
                  1, 11, 6, 11, 8, 11, 13, 11,
                  3, 13, 4, 13, 5, 13, 9, 13, 10, 13, 11, 13];
    pulsar = translateCoordListByXY(pulsar, col, row);
    universe.set_cells_by_coords(pulsar);
};

const PRIMARY_MOUSE_BUTTON = 0;
const SECONDARY_MOUSE_BUTTON = 2;
let primaryMouseBtnDown = false;
canvas.addEventListener("click", event => {
    if (event.button === PRIMARY_MOUSE_BUTTON) {
        toggleCellAtClientXY(event.clientX, event.clientY);
    }
    drawCells();
});
canvas.addEventListener("contextmenu", event => {
    event.preventDefault();
    if (event.button === SECONDARY_MOUSE_BUTTON) {
        if (event.shiftKey) {
            makePulsarAtClientXY(event.clientX, event.clientY);
        } else {
            makeSpaceShipAtClientXY(event.clientX, event.clientY);
        }
    }
    drawCells();
}, {capture: true});

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
