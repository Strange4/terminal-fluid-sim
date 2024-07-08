use criterion::{criterion_group, criterion_main, Criterion};
use terminal_fluid_sim::FluidSim;

pub fn sim_bench(c: &mut Criterion) {
    let mut sim = FluidSim::new(100, 100, 1000.0);
    c.bench_function("simulate steps", |b| {
        b.iter(|| {
            sim.next_step();
        })
    });
}

criterion_group!(benches, sim_bench);

criterion_main!(benches);
