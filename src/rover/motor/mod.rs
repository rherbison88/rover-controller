use serde::{Serialize, Deserialize};

mod wheel;

#[derive(Serialize, Debug)]
pub struct MotorCommand<'a> {
    name: &'a str,
    pub voltage: f64
}

#[derive(Debug)]
pub struct Motor {
    name: String,
    kv_rating: f64,
    current_rating: f64,
    wheel: wheel::Wheel,
    ground_speed_to_voltage_ratio: f64,
    fixed_speed_power_ratio: f64
}


#[derive(Deserialize, Debug)]
pub struct MotorBuilder {
    name: String,
    kv_rating: f64,
    current_rating: f64,
    wheel: wheel::Wheel,
}

impl MotorBuilder {
    pub fn build(self, fixed_speed_power_ratio: f64) -> Motor {
        let v1 = self.wheel.ground_speed_to_rpm(100.0) / self.kv_rating;
        let v2 = self.wheel.ground_speed_to_rpm(200.0) / self.kv_rating;
        let ground_speed_to_voltage_ratio = 100.0 / (v2 - v1);

        Motor {
            name: self.name,
            kv_rating: self.kv_rating,
            current_rating: self.current_rating,
            wheel: self.wheel,
            ground_speed_to_voltage_ratio,
            fixed_speed_power_ratio,
        }
    }

    pub fn power_at_ground_speed(&self, speed: f64) -> f64 {
        (self.wheel.ground_speed_to_rpm(speed) / self.kv_rating) * self.current_rating
    }
}

impl Motor {
    pub fn current_rating_get(&self) -> f64 {
        self.current_rating
    }

    pub fn fixed_speed_power_ratio_get(&self) -> f64 {
        self.fixed_speed_power_ratio
    }

    pub fn ground_speed_get(&self, volts: f64) -> f64 {
        volts * self.ground_speed_to_voltage_ratio
    }

    pub fn ground_speed_to_voltage(&self, speed: f64) -> f64 {
        speed / self.ground_speed_to_voltage_ratio
    }

    pub fn ground_speed_set(&self, speed: f64) -> MotorCommand {
        MotorCommand {
            name: &self.name,
            voltage: self.ground_speed_to_voltage(speed)
        }
    }
}

