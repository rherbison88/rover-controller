use serde::Deserialize;

mod motor;
mod battery;
mod solar_panel;

#[derive(Deserialize, Debug)]
pub struct Rover {
    motors: Vec<motor::Motor>,
    batteries: Vec<battery::Battery>,
    solar_panels: Vec<solar_panel::SolarPanel>,
}
