use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WheelPosition {
    #[allow(unused)]
    x: f64,
    #[allow(unused)]
    y: f64,
}

#[derive(Deserialize, Debug)]
pub struct Wheel {
    diameter: f64,
    #[allow(unused)]
    position: WheelPosition,
    gear_ratio: f64,
}

impl Wheel {
    fn circumference_get(&self) -> f64 {
        std::f64::consts::PI * self.diameter
    }

    pub fn ground_speed_to_rpm(&self, speed: f64) -> f64 {
        (speed * self.gear_ratio * 60.0) / self.circumference_get()
    }
}
