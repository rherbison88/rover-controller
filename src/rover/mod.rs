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
    limiting_motor: (usize, f64),
}

#[derive(Deserialize, Debug)]
pub struct RoverBuilder {
    motors: Vec<motor::Motor>,
    batteries: Vec<battery::Battery>,
    solar_panels: Vec<solar_panel::SolarPanel>,
}

impl RoverBuilder {
    pub fn build(self) -> Rover {
        const SEARCH_FIXED_SPEED: f64 = 100.0;
        struct LimitingMotorSearch {
            idx: isize, // reference to the individual motor
            speed: f64, // ground speed produced from fixed voltage passed during search
            total_power: f64, // power used by all motors searched for a fixed speed
            idx_found: usize
        }

        let searcher_init =
            LimitingMotorSearch {
                idx: -1,
                speed: f64::MAX,
                total_power: 0.0,
                idx_found: 0
            };

        let found: LimitingMotorSearch = self.motors.iter().fold(searcher_init, |acc, m| {
            /* loop through using a fixed voltage,
             * and find the motor which produces the lowest ground speed */
            let idx = acc.idx + 1;
            let total_power = acc.total_power + (m.current_rating_get() * m.ground_speed_to_voltage(SEARCH_FIXED_SPEED));
            let speed = m.ground_speed_get(10.0);
            if speed < acc.speed {
                LimitingMotorSearch {
                    idx,
                    speed,
                    total_power,
                    idx_found: idx.try_into().unwrap()
                }
            } else {
                LimitingMotorSearch {
                    idx,
                    speed: acc.speed,
                    total_power,
                    idx_found: acc.idx_found
                }
            }
        });

        let fixed_speed_power = self.motors[found.idx_found].current_rating_get() * self.motors[found.idx_found].ground_speed_to_voltage(SEARCH_FIXED_SPEED);
        let limiting_pow_ratio = fixed_speed_power / found.total_power;

        Rover {
            motors: self.motors,
            batteries: self.batteries,
            solar_panels: self.solar_panels,
            limiting_motor: (found.idx_found, limiting_pow_ratio)
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

    fn max_speed_get(&self, voltage: f64) -> f64 {
        /* return the lowest max speed of all motor/wheel pairs */
        self.motors[self.limiting_motor.0].ground_speed_get(voltage)
    }

    fn power_to_speed(&self, power: f64) -> f64 {
        let limiting_motor = &self.motors[self.limiting_motor.0];
        let v = (self.limiting_motor.1 * power) / limiting_motor.current_rating_get();
        limiting_motor.ground_speed_get(v)
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
