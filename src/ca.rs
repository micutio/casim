//! Module for cellular automata

use std::mem;

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
        debug!("creating simulation");
        Simulation {
            width,
            height,
            state,
            buffer,
            transition,
            neighborhood,
        }
    }

    // TODO: Return error if `cells` doesn't match the width and height parameter.
    pub fn from_cells(
        width: i32,
        height: i32,
        transition: T,
        neighborhood: N,
        cells: Vec<C>,
    ) -> Self {
        let state = cells;
        let buffer = vec![C::default(); state.len()];
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

fn _idx_to_coord(width: usize, idx: usize) -> (i32, i32) {
    let x = idx % width;
    let y = idx / width;
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

// test cases: matrix with width=4
// - (2,1) => 6
// - (3,3) => 15
// - (1,5) => 21

#[test]
fn test_coord_to_idx() {
    let coord = (3, 3);
    let idx = coord_to_idx(4, coord.0, coord.1);
    assert!(idx == 15);
}

#[test]
fn test_idx_to_coord() {
    let idx = 15;
    let coord = _idx_to_coord(4, idx);
    assert!(coord == (3, 3));
}

#[test]
fn test_roundtrip_idx_coords() {
    for idx in 0..9_999 {
        for width in 1..10_000 {
            let coord = _idx_to_coord(width, idx);
            let new_idx = coord_to_idx(width as i32, coord.0, coord.1);
            // println!(
            //     "idx: {0}, new_idx: {1}, width: {2}, (coord=({3}, {4}))",
            //     idx, new_idx, width, coord.0, coord.1
            // );
            assert!(idx == new_idx);
        }
    }
}

#[test]
fn test_roundtrip_coords_idx() {
    for width in 1..99 {
        for y in 0..99 {
            for x in 0..(width - 1) {
                let idx = coord_to_idx(width, x, y);
                let new_coord = _idx_to_coord(width as usize, idx);
                println!(
                    "(x, y)=({0}, {1}), idx={2}, new_coord=({3}, {4}), width={5}",
                    x, y, idx, new_coord.0, new_coord.1, width
                );
                assert!(new_coord.0 == x && new_coord.1 == y);
            }
        }
    }
}
