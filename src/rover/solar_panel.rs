use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SolarPanel {
    efficiency: f64,
    area: f64,
}
