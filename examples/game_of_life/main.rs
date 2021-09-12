//! Example of a simple game of life CA

use casim::ca::{Neighborhood, Simulation, VON_NEUMAN_NEIGHBORHOOD};

fn main() {
    let trans_fn = |cell: &mut bool, neigh_it: Neighborhood<bool>| {
        let true_count = neigh_it.into_iter().filter(|c| **c == true).count();

        if true_count > 2 {
            *cell = true;
        }
    };

    let mut gol = Simulation::new(10, 10, trans_fn, VON_NEUMAN_NEIGHBORHOOD);

    gol.step();
}
