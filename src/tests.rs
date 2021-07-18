#[cfg(test)]
mod tests {
    use crate::ca::Simulation;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

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
        let von_neuman = |x: i32, y: i32| {
            vec![
                (x - 1, y),
                (x, y - 1),
                (x - 1, y - 1),
                (x + 1, y),
                (x, y + 1),
                (x + 1, y + 1),
            ]
        };

        let gol = Simulation::new(10, 10, trans_fn, von_neuman);

        for c in gol.cells() {
            assert!(!c)
        }
    }
}
