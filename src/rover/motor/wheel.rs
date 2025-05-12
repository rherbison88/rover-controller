use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WheelPosition {
    x: f64,
    y: f64
}

#[derive(Deserialize, Debug)]
pub struct Wheel {
    diameter: f64,
    position: WheelPosition,
    gear_ratio: f64,
}

