use casim::ca::{von_neuman, Simulation};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn gol_benchmark(c: &mut Criterion) {
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

        if trues >= falses {
            *cell = true;
        }
        if falses > trues {
            *cell = false;
        }
    };

    let cells = vec![false, true, false, true, false, true, false, true, false];

    assert!(cells.len() == 9);

    let mut sim = Simulation::from_cells(3, 3, trans_fn, von_neuman, cells);

    c.bench_function("gol", |b| b.iter(|| sim.step_until(100)));
}

criterion_group!(benches, gol_benchmark);
criterion_main!(benches);
