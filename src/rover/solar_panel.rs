use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SolarPanel {
    efficiency: f64,
    area: f64,
}

impl SolarPanel {
    pub fn power_per_irradiance_get(&self) -> f64 {
        self.efficiency * self.area
    }

    pub fn power_output_get(&self, irradiance: f64) -> f64 {
        irradiance * self.power_per_irradiance_get()
    }
}
