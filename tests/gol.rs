//! For a reference on building benchmarks look at:
//! https://bheisler.github.io/criterion.rs/book/getting_started.html

#[cfg(test)]
use casim::ca::{Neighborhood, Simulation, VON_NEUMAN_NEIGHBORHOOD};

/// Create a simple cellular automaton and flip all cells
#[test]
fn game_of_life() {
    let mut gol = create_ca();
    gol.step();

    let cells_post_ca = vec![false, true, false, true, true, true, false, true, false];

    dbg!("cells in ca: {:?}", &gol.cells());
    dbg!("cells to compare: {:?}", &cells_post_ca);

    assert!(gol.cells() == cells_post_ca);
}

fn create_ca() -> Simulation<bool> {
    let trans_fn = |cell: &mut bool, neigh_it: Neighborhood<bool>| {
        let true_count = neigh_it.into_iter().filter(|c| **c).count();
        println!("true count: {}", true_count);
        if true_count > 2 {
            *cell = true;
        }
    };

    let cells = vec![false, true, false, true, false, true, false, true, false];

    assert!(cells.len() == 9);

    Simulation::from_cells(3, 3, trans_fn, VON_NEUMAN_NEIGHBORHOOD, cells)
}
