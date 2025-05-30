use log::debug;
use serde::Deserialize;
use std::error::Error;

mod exercises;
mod rover;

const MARTIAN_DAY_S: f64 = 3600.0 * 24.0 + 60.0 * 37.0 + 22.0;

#[derive(Deserialize)]
struct HealthResp {
    status: String,
}

#[derive(Debug)]
struct Health {
    status: Result<(), Box<dyn Error>>,
}

impl From<HealthResp> for Health {
    fn from(value: HealthResp) -> Self {
        let status = if value.status.as_str() == "Ok" {
            Ok(())
        } else {
            Err("server health not \"Ok\"".into())
        };
        Health { status }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let client = reqwest::Client::new();

    let health: Health = client
        .get("http://localhost:8080/health")
        .send()
        .await?
        .json::<HealthResp>()
        .await?
        .into();
    debug!("{health:#?}\n\n");

    if health.status.is_err() {
        /* we will return an error before getting here... */
        panic!("Server is not running");
    }

    let exercises = client
        .get("http://localhost:8080/exercises")
        .send()
        .await?
        .json::<exercises::Exercises>()
        .await?;

    debug!("{exercises:#?}\n\n");

    let rover_builder = client
        .get("http://localhost:8080/rover/config")
        .send()
        .await?
        .json::<rover::RoverBuilder>()
        .await?;

    debug!("{rover_builder:#?}\n\n");

    let rover = rover_builder.build();

    let distance_cmd = rover.distance_travel(exercises.fixed_distance.value);
    println!("Fixed Distance Test: trying rover command = \n{distance_cmd:#?}");
    let resp = client
        .post("http://localhost:8080/verify/fixed_distance")
        .json(&distance_cmd)
        .send()
        .await?
        .json::<exercises::VerifyResponse>()
        .await?;
    println!("{resp:#?}\n");

    let max_dist = rover.max_distance_get(exercises.fixed_capacity.state_of_charge);
    println!("Fixed Capacity Test: trying distance = {max_dist}");
    let resp = client
        .post("http://localhost:8080/verify/fixed_capacity")
        .json(&max_dist)
        .send()
        .await?
        .json::<exercises::VerifyResponse>()
        .await?;
    println!("{resp:#?}\n");

    let max_speed = rover.panel_only_max_speed_get(exercises.fixed_irradiance.value);
    println!("Fixed Irradiance Test: trying speed = {max_speed}");
    let resp = client
        .post("http://localhost:8080/verify/fixed_irradiance")
        .json(&max_speed)
        .send()
        .await?
        .json::<exercises::VerifyResponse>()
        .await?;
    println!("{resp:#?}\n");

    let daily_dist = daily_distance_calc(&rover, exercises.variable_irradiance.peak_value);
    println!("Variable Irradiance Test: trying distance = {daily_dist}");
    let resp = client
        .post("http://localhost:8080/verify/variable_irradiance")
        .json(&daily_dist)
        .send()
        .await?
        .json::<exercises::VerifyResponse>()
        .await?;
    println!("{resp:#?}\n");

    Ok(())
}

fn daily_distance_calc(rover: &rover::Rover, peak_irradiance: f64) -> f64 {
    /* variable solar irradiance strategy:
     *
     * The max distance which can be travelled can be determined by:
     * 1. find the total solar energy generated by the solar panels for a day
     *
     *     To determine energy (time integral of power):
     *         angular_frequency = 2*pi/MARTIAN_DAY_S
     *         energy = -(peak_power)/angular_frequency*cos(t_end*angular_frequency) +
     *                   (peak_power)/angular_frequency*cos(t_start*angular_frequency)
     *     where t_start = 0, and t_end = start of night (1/2 day from start of morning)
     *
     * 2. reshape this energy such that it represents a rectangle with height = saturated power,
     *    and length = amount of time which we can travel at saturated power.
     *
     * 3. calculate distance by multiplying this new time by the speed at saturated power.
     */

    let ang_freq = (std::f64::consts::PI * 2.0) / MARTIAN_DAY_S;
    let peak_power = peak_irradiance * rover.power_per_irradiance_get();
    let time_end_of_radiation = MARTIAN_DAY_S / 2.0;

    let time1_start = 0.0;
    let time1_end = time_end_of_radiation;

    /* time integral of power generated by the solar panels (providing energy) */
    let solar_energy = (-(peak_power) / ang_freq) * ((time1_end * ang_freq).cos())
        + ((peak_power) / ang_freq) * ((time1_start * ang_freq).cos());

    let new_time = solar_energy / rover.saturated_power_get();

    (new_time * rover.power_to_speed(rover.saturated_power_get())) / 1e6
}
