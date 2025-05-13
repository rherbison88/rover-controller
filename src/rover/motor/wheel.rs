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


impl Wheel {
    fn circumference_get(&self) -> f64 {
        std::f64::consts::PI * self.diameter
    }

    pub fn ground_speed_get(&self, rpm: f64) -> f64 {
        let wheel_rpm = rpm / self.gear_ratio;
        (wheel_rpm * self.circumference_get()) / 60.0
    }

    pub fn ground_speed_to_rpm(&self, speed: f64) -> f64 {
        (speed * self.gear_ratio * 60.0) / self.circumference_get()
    }
}
