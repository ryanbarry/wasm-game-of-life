mod utils;

extern crate fixedbitset;
extern crate js_sys;
extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        //web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

use fixedbitset::FixedBitSet;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // TODO: try profiling this modular arithmetic vs conditional based wrapping (maybe find that c++ code i wrote for GoL somewhere else...)
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );

                let next_cell = match (cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                };

                log!("    it becomes {:?}", next_cell);

                if cell != next_cell {
                    log!(
                        "cell[{}, {}] is changing to {}",
                        row,
                        col,
                        if next_cell { "alive" } else { "dead" }
                    );
                }

                next.set(idx, next_cell);
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        const WIDTH: u32 = 64;
        const HEIGHT: u32 = 64;

        // start with columns
        /*let cells = (0..WIDTH * HEIGHT)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();*/

        // start with a single LWSS
        /*
        let mut cells: Vec<Cell> = (0..WIDTH * HEIGHT).map(|_| Cell::Dead).collect();
        const LWSS: [[Cell; 5]; 4] = [
            [Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Alive],
            [Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead],
            [Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive],
            [
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Alive,
                Cell::Dead,
            ],
        ];
        const LWSS_START_ROW: usize = (HEIGHT / 2) as usize;
        const LWSS_START_COL: usize = (WIDTH / 2) as usize;
        for (i, row) in LWSS.iter().enumerate() {
            for (j, lwss_cell) in row.iter().enumerate() {
                let cells_x = LWSS_START_COL + j;
                let cells_y = LWSS_START_ROW + i;
                let idx = cells_y * (width as usize) + cells_x;
                cells[idx] = *lwss_cell;
            }
        }
         */

        // start with random
        let size = (WIDTH * HEIGHT) as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, js_sys::Math::random() > 0.5);
        }

        Universe {
            width: WIDTH,
            height: HEIGHT,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
        self.cells.set_range(.., false);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = FixedBitSet::with_capacity((self.width * self.height) as usize);
        self.cells.set_range(.., false);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }
}

impl Universe {
    pub fn get_cells(&self) -> &FixedBitSet {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

use std::fmt;
const ALIVE_CODE_POINT: char = '\u{25FC}'; // BLACK MEDIUM SQUARE
const DEAD_CODE_POINT: char = '\u{25FB}'; // WHITE MEDIUM SQUARE
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == 0 {
                    DEAD_CODE_POINT
                } else {
                    ALIVE_CODE_POINT
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
