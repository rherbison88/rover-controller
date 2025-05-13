use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Battery {
    capacity: f64,
    max_voltage: f64,
}

impl Battery {
    pub fn max_voltage_get(&self) -> f64 {
        self.max_voltage
    }
    pub fn capacity_get(&self) -> f64 {
        self.capacity
    }
}
