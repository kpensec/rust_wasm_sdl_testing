extern crate rand;
use sdl2::pixels::Color;
use render::RenderContext;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    time_accumulator: f32,
    time_update: f32,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn set_time_update(&mut self, d: f32) {
        self.time_update += d;
    }
    pub fn get_time_update(&self) -> f32 {
        self.time_update
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u32 {
        let mut count = 0;
        for d_row in [self.height - 1, 0, 1].iter().cloned() {
            for d_col in [self.width - 1, 0, 1].iter().cloned() {
                if d_col == 0 && d_row == 0 {
                    continue;
                }

                let neighbor_row = (row + d_row) % self.height;
                let neighbor_col = (col + d_col) % self.width;

                let idx = self.get_index(neighbor_row, neighbor_col);

                count += self.cells[idx] as u32;
            }
        }
        count
    }
    pub fn tick(&mut self, eps: f32) {
        self.time_accumulator += eps;
        if self.time_accumulator < self.time_update {
            return;
        }
        self.time_accumulator = 0.0;

        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
    pub fn new(time_update: f32) -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
        .collect();

        Universe {
            width,
            height,
            cells,
            time_accumulator: 0.0,
            time_update: time_update,
        }
    }
}

use sdl2::render::RenderTarget;

pub trait Renderer<T: RenderTarget> {
    fn render(&self, ctx: &mut RenderContext<T>);
}

pub fn get_random_color() -> Color  {
    Color::RGB(
        rand::random::<u8>(),
        rand::random::<u8>(),
        rand::random::<u8>()
    )
}
impl<T: RenderTarget> Renderer<T> for Universe {
    fn render(&self, r: &mut RenderContext<T>) {
        for i in 0..self.width {
            for j in 0..self.height {
                match self.cells[self.get_index(i,j)] {
                    Cell::Alive => r.display_cell(i as i32, j as i32, 8, get_random_color()),
                    _ => Ok(())
                }.unwrap();
            }
        }
    }
}
