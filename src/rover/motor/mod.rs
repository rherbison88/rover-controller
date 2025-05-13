use serde::{Serialize, Deserialize};

mod wheel;

#[derive(Serialize, Debug)]
pub struct MotorCommand<'a> {
    name: &'a str,
    voltage: f64
}


#[derive(Deserialize, Debug)]
pub struct Motor {
    name: String,
    kv_rating: f64,
    current_rating: f64,
    wheel: wheel::Wheel,
}

impl Motor {
    pub fn ground_speed_get(&self, volts: f64) -> f64 {
        let motor_rpm = self.kv_rating * volts;
        self.wheel.ground_speed_get(motor_rpm)
    }

    pub fn ground_speed_set(&self, speed: f64) -> MotorCommand {
        let voltage = self.wheel.ground_speed_to_rpm(speed) / self.kv_rating;
        MotorCommand {
            name: &self.name,
            voltage
        }
    }
}

