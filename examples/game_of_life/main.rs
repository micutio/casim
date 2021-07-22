//! Example of a simple game of life CA

use grid_machine::ca::{von_neuman, Simulation};

fn main() {
    let trans_fn = |cell: &mut bool, neighs: &[&bool]| {
        let mut trues: i32 = 0;
        let mut falses: i32 = 0;
        for n in neighs {
            if **n {
                trues += 1;
            } else {
                falses += 1;
            }
        }

        if trues > falses {
            *cell = true;
        }
        if falses > trues {
            *cell = false;
        }
    };

    let mut gol = Simulation::new(10, 10, trans_fn, von_neuman);

    gol.step();
}