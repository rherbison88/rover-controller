use serde::{Serialize, Deserialize};

mod motor;
mod battery;
mod solar_panel;


#[derive(Serialize, Debug)]
pub struct RoverCommand<'a> {
    duration: f64,
    motor_commands: Vec<motor::MotorCommand<'a>>
}

#[derive(Debug)]
pub struct Rover {
    motors: Vec<motor::Motor>,
    batteries: Vec<battery::Battery>,
    solar_panels: Vec<solar_panel::SolarPanel>,
    limiting_motor_idx: usize
}

#[derive(Deserialize, Debug)]
pub struct RoverBuilder {
    motors: Vec<motor::MotorBuilder>,
    batteries: Vec<battery::Battery>,
    solar_panels: Vec<solar_panel::SolarPanel>,
}

impl RoverBuilder {
    pub fn build(self) -> Rover {
        const SEARCH_FIXED_SPEED: f64 = 100.0;

        /* find the total power for a given fixed ground travel speed */
        let total_power = self.motors.iter().fold(0.0, |acc, m| acc + m.power_at_ground_speed(SEARCH_FIXED_SPEED));

        /* use it to find the power ratio of each motor at a fixed ground travel speed, and use
         * that value to build the MotorBuilder objects into Motor objects */
        let built_motors: Vec<motor::Motor> = self.motors.into_iter().map(|m| {
            let power_ratio = m.power_at_ground_speed(SEARCH_FIXED_SPEED) / total_power;
            m.build(power_ratio)
        }).collect();

        /* Find the index of the limiting motor. The limiting motor is the motor with the highest
         * voltage to ground speed ratio, as this motor will limit our top speed based on the max
         * battery votage */
        let (_, idx, _) = built_motors.iter().fold((-1, 0, 0.0), |acc, m| {
            let idx = acc.0 + 1;
            let v = m.ground_speed_to_voltage(SEARCH_FIXED_SPEED);
            if v > acc.2 { (idx, idx.try_into().unwrap(), v) } else { (idx, acc.1, acc.2) }
        });

        Rover {
            motors: built_motors,
            batteries: self.batteries,
            solar_panels: self.solar_panels,
            limiting_motor_idx: idx
        }
    }
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
        let power = self.solar_panels.iter().fold(0.0, |acc, sp| acc + sp.power_output_get(irradiance));
        self.power_to_speed(power)
    }

    pub fn saturated_power_get(&self) -> f64 {
        let max_v = self.batteries[0].max_voltage_get();
        self.motors.iter().fold(0.0, |acc, m| acc + m.ground_speed_to_power(self.max_speed_get(max_v)))
    }

    pub fn power_per_irradiance_get(&self) -> f64 {
        self.solar_panels.iter().fold(0.0, |acc, sp| acc + sp.power_per_irradiance_get())
    }

    fn max_speed_get(&self, voltage: f64) -> f64 {
        /* return the lowest max speed of all motor/wheel pairs */
        self.motors[self.limiting_motor_idx].ground_speed_get(voltage)
    }

    pub fn power_to_speed(&self, power: f64) -> f64 {
        let limiting_motor = &self.motors[self.limiting_motor_idx];
        let v = (limiting_motor.fixed_speed_power_ratio_get() * power) / limiting_motor.current_rating_get();
        let max_v = self.batteries[0].max_voltage_get();
        let v = if v < max_v { v } else { max_v };
        limiting_motor.ground_speed_get(v)
    }

    fn max_time_at_speed(&self, state_of_charge: f64, speed: f64) -> f64 {
        /* time until batteries drain (s) =
         *     ( battery_capacity (Wh) / sum of motor powers (W) ) * 3600 (secs/h) */
        let power_sum = self.motors.iter().fold(0.0, |acc, m| {
            acc + (m.ground_speed_to_voltage(speed) * m.current_rating_get())
        });

        (self.batt_capacity_get() * 36.0 * state_of_charge) / power_sum
    }


    fn speed_set(&self, speed: f64) -> Vec<motor::MotorCommand> {
        self.motors.iter().map(|m| m.ground_speed_set(speed)).collect()
    }

    fn batt_voltage_get(&self, state_of_charge: f64) -> f64 {
        (self.batteries[0].max_voltage_get() * state_of_charge) / 100.0
    }

    pub fn batt_capacity_get(&self) -> f64 {
        self.batteries.iter().fold(0.0, |acc, b| acc + b.capacity_get())
    }
}
