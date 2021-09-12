//! Ensures that cells initialised with coordinates are at the correct place in the state vector.

use casim::ca::{idx_to_coord, Neighborhood, Simulation, VON_NEUMAN_NEIGHBORHOOD};
use criterion::{criterion_group, criterion_main, Criterion};

/// Create a grid of cells with coordinates and for any given cell test whether the coordinates of
/// neighbor cells line up with it.
pub fn indexing_benchmark(c: &mut Criterion) {
    c.bench_function("indexing", |b| {
        b.iter(|| {
            for height in 0..10 {
                for width in 0..10 {
                    let mut test_ca = create_ca(width, height);
                    test_ca.step();
                }
            }
        })
    });
}

#[derive(Clone, Copy, Default, PartialEq, std::fmt::Debug)]
struct LocatableCell {
    x: i32,
    y: i32,
}
fn create_ca(width: i32, height: i32) -> Simulation<LocatableCell> {
    let trans_fn = |cell: &mut LocatableCell, neigh_it: Neighborhood<LocatableCell>| {
        let mut found_neighbors: Vec<(i32, i32)> = Vec::new();
        for n in neigh_it.into_iter() {
            if !((cell.x == n.x && (cell.y == n.y - 1 || cell.y == n.y + 1))
                || (cell.y == n.y && (cell.x == n.x - 1 || cell.x == n.x + 1)))
            {
                // println!(
                //     "this cell: ({},{}), neighbour: ({},{})",
                //     cell.x, cell.y, n.x, n.y
                // );
                assert!(false);
            }
            if found_neighbors.contains(&(n.x, n.y)) {
                assert!(false);
            }
            found_neighbors.push((n.x, n.y));
        }
    };

    let cells = (0..width * height)
        .into_iter()
        .map(|idx| {
            let coord = idx_to_coord(width as usize, idx as usize);
            LocatableCell {
                x: coord.0,
                y: coord.1,
            }
        })
        .collect();

    Simulation::from_cells(width, height, trans_fn, VON_NEUMAN_NEIGHBORHOOD, cells)
}

criterion_group!(benches, indexing_benchmark);
criterion_main!(benches);
