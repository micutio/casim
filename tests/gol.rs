//! For a reference on building benchmarks look at:
//! https://bheisler.github.io/criterion.rs/book/getting_started.html

#[cfg(test)]
use casim::ca::{von_neuman, Simulation};

/// Create a simple cellular automaton and flip all cells
#[test]
fn game_of_life() {
    let mut trans_fn = |cell: &mut bool, neighs: &[&bool]| {
        let mut trues: i32 = 0;
        let mut falses: i32 = 0;
        for n in neighs {
            if **n {
                trues += 1;
            } else {
                falses += 1;
            }
        }

        if trues >= falses {
            *cell = true;
        }
        if falses > trues {
            *cell = false;
        }
    };

    let cells = vec![false, true, false, true, false, true, false, true, false];

    assert!(cells.len() == 9);

    let mut gol = Simulation::from_cells(3, 3, &mut trans_fn, &von_neuman, cells);

    gol.step();

    let cells_post_ca = vec![true, false, true, false, true, false, true, false, true];

    println!("TEST OUTPUT TEST");

    dbg!("cells in ca: {:?}", &gol.cells());
    dbg!("cells to compare: {:?}", &cells_post_ca);

    assert!(gol.cells() == cells_post_ca);
}
