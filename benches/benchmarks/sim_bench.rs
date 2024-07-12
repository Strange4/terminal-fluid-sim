use criterion::{criterion_group, Criterion};
use terminal_fluid_sim::{FluidSim, SimConfig};

pub fn sim_bench(c: &mut Criterion) {
    let config = SimConfig::default();
    let mut sim = FluidSim::new(100, 100, config);
    c.bench_function("simulate steps", |b| {
        b.iter(|| {
            sim.next_step();
        })
    });
}

criterion_group!(benches, sim_bench);
