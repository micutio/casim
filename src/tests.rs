#[cfg(test)]
mod tests {
    use crate::ca::{von_neuman, Simulation};

    #[test]
    fn game_of_life() {
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

        let cells = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, true, false, false, false, false, false,
            false, false, false, false, true, false, true, false, false, false, false, false,
            false, false, false, false, false, true, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false,
        ];

        assert!(cells.len() == 100);

        let mut gol = Simulation::from_cells(10, 10, trans_fn, von_neuman, cells);

        // for c in gol.cells() {
        //     assert!(!c)
        // }

        gol.step();

        let cells_post_ca = vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, true, true, false, false, false, false, false,
            false, false, false, false, true, true, true, false, false, false, false, false, false,
            false, false, false, false, true, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ];

        println!("TEST OUTPUT TEST");

        dbg!("cells in ca: {:?}", &gol.cells());
        dbg!("cells to compare: {:?}", &cells_post_ca);

        assert!(gol.cells() == cells_post_ca);
    }
}
