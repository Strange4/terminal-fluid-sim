use std::{io::Error, time::Duration};

use crate::fluid_sim::simulator::{
    tests::{read_file, FluidSimData},
    FluidSim,
};

#[test]
fn known_steps() -> Result<(), Error> {
    let json = read_file("step-data.json")?;
    let data: FluidSimData = serde_json::from_str(&json)?;

    let mut sim = FluidSim::new(7, 7, 1000.0);

    sim.step_through(Duration::from_secs_f64(1.0 / 60.0));
    sim.step_through(Duration::from_secs_f64(1.0 / 60.0));
    sim.step_through(Duration::from_secs_f64(1.0 / 60.0));
    sim.step_through(Duration::from_secs_f64(1.0 / 60.0));

    let flat: Vec<f32> = sim.horizontal_values.into_iter().flatten().collect();

    assert_eq!(data.horizontal_velocity, flat);
    Ok(())
}
