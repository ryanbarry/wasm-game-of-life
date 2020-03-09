mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
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

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        const width: u32 = 256;
        const height: u32 = 256;

        /*let cells = (0..width * height)
        .map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();*/
        let mut cells: Vec<Cell> = (0..width * height).map(|_| Cell::Dead).collect();
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
        const LWSS_START_ROW: usize = (height / 2) as usize;
        const LWSS_START_COL: usize = (width / 2) as usize;
        for (i, row) in LWSS.iter().enumerate() {
            for (j, lwss_cell) in row.iter().enumerate() {
                let cells_x = LWSS_START_COL + j;
                let cells_y = LWSS_START_ROW + i;
                let idx = cells_y * (width as usize) + cells_x;
                cells[idx] = *lwss_cell;
            }
        }

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

use std::fmt;
const ALIVE_CODE_POINT: char = '\u{25FC}'; // BLACK MEDIUM SQUARE
const DEAD_CODE_POINT: char = '\u{25FB}'; // WHITE MEDIUM SQUARE
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {
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
