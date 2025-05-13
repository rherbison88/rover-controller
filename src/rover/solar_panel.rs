use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SolarPanel {
    efficiency: f64,
    area: f64,
}

impl SolarPanel {
    pub fn power_output_get(&self, irradiance: f64) -> f64 {
        irradiance * self.efficiency * self.area
    }
}
