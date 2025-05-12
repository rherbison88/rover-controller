use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Battery {
    capacity: f64,
    max_voltage: f64,
}
