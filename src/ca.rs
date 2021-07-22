//! Module for cellular automata

use std::mem;

use crate::modulus;

/// C = Cell
///     - data type of the cell
/// T = Transition
///     - for a given cell and neighborhood, define how the new cell state is computed
///     - either clone the cell or create a diff
/// N = Neighborhood
///     - for a given cell (position) get all neighboring cells (positions)
///     - Fn(i32, i32) -> [(i32, i32)]
pub struct Simulation<C, T, N> {
    width: i32,
    height: i32,
    state: Vec<C>,
    buffer: Vec<C>,
    transition: T,
    neighborhood: N,
}

/// T applies a function to Cell of buffer 1 and neighborhood and then puts a clone of the cell with the new state in buffer 2
impl<C, T, N> Simulation<C, T, N>
where
    C: Clone + Default,
    T: FnMut(&mut C, &[&C]),
    N: Fn(i32, i32) -> Vec<(i32, i32)>,
{
    pub fn new(width: i32, height: i32, transition: T, neighborhood: N) -> Self {
        let capacity: usize = (width * height) as usize;
        let state = vec![C::default(); capacity];
        let buffer = vec![C::default(); capacity];
        debug!("creating simuation");
        Simulation {
            width,
            height,
            state,
            buffer,
            transition,
            neighborhood,
        }
    }

    /// Perform one simulation step.
    pub fn step(&mut self) {
        // Manipulate the internal state of a cell the `state` grid by iterating over the cells at
        // the neighborhood coordinates in the `buffer` grid.
        for x in 0..self.width {
            for y in 0..self.height {
                let w = self.width;
                let buf_ref = &self.buffer;
                let state_ref = &mut self.state;
                let neighbors: Vec<&C> = (self.neighborhood)(x, y)
                    .iter()
                    .map(|(i, j)| &buf_ref[coord_to_idx(w, *i, *j)])
                    .collect();
                (self.transition)(&mut state_ref[coord_to_idx(w, x, y)], &neighbors)
            }
        }

        // Swap the assignments of `state` and `buffer` to "update the grid", so to speak.
        mem::swap(&mut self.state, &mut self.buffer)
    }

    pub fn step_until(&mut self, step_count: i32) {
        for _ in 0..step_count {
            self.step();
        }
    }

    pub fn cells(&self) -> &[C] {
        &self.state
    }
}

fn coord_to_idx(width: i32, x: i32, y: i32) -> usize {
    (y * width + x) as usize
}

fn _idx_to_coordinate(idx: usize, width: usize) -> (i32, i32) {
    let x = modulus(idx, width);
    let y = idx % width;
    (x as i32, y as i32)
}

pub fn von_neuman(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![
        (x - 1, y),
        (x, y - 1),
        (x - 1, y - 1),
        (x + 1, y),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}
