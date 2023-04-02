//! Module for cellular automata

use std::mem;

pub type NeighborhoodFunction<T> = dyn FnMut(&mut T, Neighborhood<T>);

/// C = Cell
///     - data type of the cell
/// T = Transition
///     - for a given cell and neighborhood, define how the new cell state is computed
///     - either clone the cell or create a diff
/// N = Neighborhood
///     - for a given cell (position) get all neighboring cells (positions)
///     - Fn(i32, i32) -> [(i32, i32)]
pub struct Simulation<C: Send> {
    width:        i32,
    height:       i32,
    transition:   Box<NeighborhoodFunction<C>>,
    neighborhood: &'static [(i32, i32)],
    state:        Vec<C>,
    buffer:       Vec<C>,
}

/// T applies a function to Cell of buffer 1 and neighborhood and then puts a clone of the cell with
/// the new state in buffer 2
impl<C> Simulation<C>
where
    C: Send + Clone + Default + std::fmt::Debug,
{
    pub fn new(
        width: i32,
        height: i32,
        trans_fn: impl FnMut(&mut C, Neighborhood<C>) + 'static,
        neighborhood: &'static [(i32, i32)],
    ) -> Self {
        let capacity: usize = (width * height) as usize;
        let state = vec![C::default(); capacity];
        let buffer = vec![C::default(); capacity];
        debug!("creating simulation");
        Simulation {
            width,
            height,
            transition: Box::new(trans_fn),
            neighborhood,
            state,
            buffer,
        }
    }

    // TODO: Return error if `cells` doesn't match the width and height parameter.
    pub fn from_cells(
        width: i32,
        height: i32,
        trans_fn: impl FnMut(&mut C, Neighborhood<C>) + 'static,
        neighborhood: &'static [(i32, i32)],
        cells: Vec<C>,
    ) -> Self {
        Simulation {
            width,
            height,
            transition: Box::new(trans_fn),
            neighborhood,
            state: cells.clone(),
            buffer: cells,
        }
    }

    /// Perform one simulation step.
    pub fn step(&mut self) {
        // Manipulate the internal state of a cell the `buffer` grid by iterating over the cells at
        // the neighborhood coordinates in the `state` grid.
        let buffr_ref = &mut self.buffer;
        let state_ref = &self.state;

        for idx in 0..buffr_ref.len() {
            // Before we perform the transition update the cell state because if the transition
            // does not change the cell it is in danger of becoming outdated.
            // This is not nice but I don't have a better idea right now.
            buffr_ref[idx] = state_ref[idx].clone();
            // perform transition
            let n = Neighborhood::new(self.neighborhood, (self.width, self.height), idx, state_ref);
            (self.transition)(&mut buffr_ref[idx], n);
        }
        // Swap the assignments of `state` and `buffer` to "update the grid", so to speak.
        mem::swap(&mut self.state, &mut self.buffer);
    }

    pub fn step_until(&mut self, step_count: i32) {
        for _ in 0..step_count {
            self.step();
        }
    }

    #[must_use]
    pub fn cells(&self) -> &[C] {
        &self.state
    }
}

#[must_use]
pub const fn coord_to_idx(width: i32, x: i32, y: i32) -> usize {
    (y * width + x) as usize
}

#[must_use]
pub const fn idx_to_coord(width: usize, idx: usize) -> (i32, i32) {
    let x = idx % width;
    let y = idx / width;
    (x as i32, y as i32)
}

pub static VON_NEUMAN_NEIGHBORHOOD: &[(i32, i32); 4] = &[(-1, 0), (0, -1), (1, 0), (0, 1)];

pub struct Neighborhood<'a, C: Send> {
    count:     usize,
    bounds:    &'a [(i32, i32)],
    ca_bounds: (i32, i32),
    cell_idx:  usize,
    buffer:    &'a [C],
}

impl<'a, C> Neighborhood<'a, C>
where
    C: Send + Clone + Default + std::fmt::Debug,
{
    const fn new(
        bounds: &'a [(i32, i32)],
        ca_bounds: (i32, i32),
        cell_idx: usize,
        buffer: &'a [C],
    ) -> Self {
        Neighborhood {
            count: 0,
            bounds,
            ca_bounds,
            cell_idx,
            buffer,
        }
    }
}

// Implement `Iterator` for `Neighborhood`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl<'a, C> Iterator for Neighborhood<'a, C>
where
    C: Send + Clone + Default + std::fmt::Debug,
{
    // We can refer to this type using Self::Item
    type Item = &'a C;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count != self.bounds.len() {
            while self.count < self.bounds.len() {
                let cell = idx_to_coord(self.ca_bounds.0 as usize, self.cell_idx);
                let x = self.bounds[self.count].0 + cell.0;
                let y = self.bounds[self.count].1 + cell.1;

                self.count += 1;

                if x >= 0 && x < self.ca_bounds.0 && y >= 0 && y < self.ca_bounds.1 {
                    return Some(&self.buffer[coord_to_idx(self.ca_bounds.0, x, y)]);
                }
            }
        }
        None
    }
}

#[test]
fn test_roundtrip_idx_coords() {
    for idx in 0..9_999 {
        for width in 1..10_000 {
            let coord = idx_to_coord(width, idx);
            let new_idx = coord_to_idx(width as i32, coord.0, coord.1);
            assert!(idx == new_idx);
        }
    }
}

#[test]
fn test_roundtrip_coords_idx() {
    for width in 1..49 {
        for y in 0..49 {
            for x in 0..(width - 1) {
                let idx = coord_to_idx(width, x, y);
                let new_coord = idx_to_coord(width as usize, idx);
                assert!(new_coord.0 == x && new_coord.1 == y);
            }
        }
    }
}
