use casim::ca::{Neighborhood, Simulation, VON_NEUMAN_NEIGHBORHOOD};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn gol_benchmark(c: &mut Criterion) {
    let mut sim = create_ca();

    c.bench_function("gol", |b| b.iter(|| sim.step_until(100)));
}

fn create_ca() -> Simulation<bool> {
    let trans_fn = |cell: &mut bool, neigh_it: Neighborhood<bool>| {
        let true_count = neigh_it.into_iter().filter(|c| **c).count();

        if true_count > 2 {
            *cell = true;
        }
    };

    let cells = vec![false, true, false, true, false, true, false, true, false];

    assert!(cells.len() == 9);

    Simulation::from_cells(3, 3, trans_fn, VON_NEUMAN_NEIGHBORHOOD, cells)
}

criterion_group!(benches, gol_benchmark);
criterion_main!(benches);
