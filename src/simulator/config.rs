#[derive(Clone)]
pub struct SimConfig {
    /// gravity of the simulation, set to 0 for no gravity
    pub gravity: f32,

    /// wind speed, must be above 0
    pub wind_speed: f32,

    /// some height percentage of the screen [0,1] inclusive
    pub smoke_size: f32,

    /// density of the sim
    pub density: f32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            gravity: 0.0,
            wind_speed: 50.0,
            smoke_size: 0.25,
            density: 1000.0,
        }
    }
}
