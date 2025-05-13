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
        let max_speed = self.max_speed_get();
        let duration = dist_mm / self.max_speed_get();
        let motor_commands = self.speed_set(max_speed);

        RoverCommand {
            duration,
            motor_commands
        }
    }

    fn max_speed_get(&self) -> f64 {
        /* return the lowest max speed of all motor/wheel pairs */
        self.motors.iter().fold(f64::MAX, |min, m| {
            let speed = m.ground_speed_get(self.batt_voltage_get());
            if speed < min { speed } else { min }
        })
    }

    fn speed_set(&self, speed: f64) -> Vec<motor::MotorCommand> {
        self.motors.iter().map(|m| m.ground_speed_set(speed)).collect()
    }

    fn batt_voltage_get(&self) -> f64 {
        self.batteries.iter().fold(0.0, |acc, b| acc + b.voltage_get()) / (self.batteries.len() as f64)
    }
}







// "{\"motors\":[{\"name\":\"Motor_A\",\"kv_rating\":68.59303,\"current_rating\":19.722855,\"wheel\":{\"diameter\":162.31349,\"position\":{\"x\":134.66698,\"y\":-456.30334},\"gear_ratio\":9.266257}},{\"name\":\"Motor_B\",\"kv_rating\":74.504074,\"current_rating\":12.5037,\"wheel\":{\"diameter\":218.0736,\"position\":{\"x\":468.99258,\"y\":133.21674},\"gear_ratio\":6.0085225}},{\"name\":\"Motor_C\",\"kv_rating\":44.865322,\"current_rating\":19.862717,\"wheel\":{\"diameter\":230.55501,\"position\":{\"x\":-180.12427,\"y\":5.5152283},\"gear_ratio\":9.457709}},{\"name\":\"Motor_D\",\"kv_rating\":45.422478,\"current_rating\":10.96563,\"wheel\":{\"diameter\":205.87985,\"position\":{\"x\":-315.42743,\"y\":128.23596},\"gear_ratio\":6.5454917}}],\"batteries\":[{\"capacity\":1238.1313,\"max_voltage\":20.596033}],\"solar_panels\":[{\"efficiency\":0.18792617,\"area\":0.5640607},{\"efficiency\":0.17570704,\"area\":1.171747},{\"efficiency\":0.19554082,\"area\":0.62622285},{\"efficiency\":0.152507,\"area\":1.2510899}]}"



