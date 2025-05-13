use serde::{Serialize, Deserialize};

mod motor;
mod battery;
mod solar_panel;

#[derive(Serialize, Debug)]
pub struct RoverCommand<'a> {
    duration: f64,
    motor_commands: Vec<motor::MotorCommand<'a>>
}

#[derive(Deserialize, Debug)]
pub struct Rover {
    motors: Vec<motor::Motor>,
    batteries: Vec<battery::Battery>,
    solar_panels: Vec<solar_panel::SolarPanel>,
}

impl Rover {
    pub fn distance_travel(&self, dist_mm: f64) -> RoverCommand {
        // distance (mm) = time (s) * speed (mm/s)
        let max_speed = self.max_speed_get(self.batt_voltage_get(100.0));
        let duration = dist_mm / max_speed;
        let motor_commands = self.speed_set(max_speed);

        RoverCommand {
            duration,
            motor_commands
        }
    }

    pub fn max_distance_get(&self, state_of_charge: f64) -> f64 {
        // distance (km) = max starting speed (mm/s) * time before discharge (s) / 1e6
        let max_speed = self.max_speed_get(self.batt_voltage_get(state_of_charge));
        let duration = self.max_time_at_speed(state_of_charge, max_speed);

        max_speed * duration / 1000000.0
    }

    pub fn panel_only_max_speed_get(&self, irradiance: f64) -> f64 {
        // speed (mm/s) = max_speed_get( combined wattage of panels (W) / combined current of motors (A) )
        let power = self.solar_panels.iter().fold(0.0, |acc, sp| acc + sp.power_output_get(irradiance));
        self.power_to_speed(power)
    }

    fn max_speed_get(&self, voltage: f64) -> f64 {
        /* return the lowest max speed of all motor/wheel pairs */
        self.motors.iter().fold(f64::MAX, |min, m| {
            let speed = m.ground_speed_get(voltage);
            if speed < min { speed } else { min }
        })
    }

    fn power_to_speed(&self, watts: f64) -> f64 {
        let fixed_speed_wattages: Vec<f64> = self.motors.iter().map(|m| {
            m.ground_speed_set(100.0).voltage * m.current_rating_get()
        }).collect();
        let motor_zero_ratio = fixed_speed_wattages[0] / fixed_speed_wattages.iter().fold(0.0, |acc, w| acc + w);
        let motor_zero_power = motor_zero_ratio * watts;
        let voltage = motor_zero_power / self.motors[0].current_rating_get();
        self.motors[0].ground_speed_get(voltage)
    }

    fn max_time_at_speed(&self, state_of_charge: f64, speed: f64) -> f64 {
        /* time until batteries drain (s) =
         *     ( battery_capacity (Wh) / sum of motor powers (W) ) * 3600 (secs/h) */
        let power_sum = self.motors.iter().fold(0.0, |acc, m| {
            acc + (m.ground_speed_to_voltage(speed) * m.current_rating_get())
        });

        let batt_capacity_sum = self.batteries.iter().fold(0.0, |acc, b| acc + b.capacity_get());
        (batt_capacity_sum * 36.0 * state_of_charge) / power_sum
    }

    fn speed_set(&self, speed: f64) -> Vec<motor::MotorCommand> {
        self.motors.iter().map(|m| m.ground_speed_set(speed)).collect()
    }

    fn batt_voltage_get(&self, state_of_charge: f64) -> f64 {
        (self.batteries[0].voltage_get() * state_of_charge) / 100.0
    }
}
