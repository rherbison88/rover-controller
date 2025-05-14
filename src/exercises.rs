use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FixedDistance {
    pub value: f64,
}

#[derive(Deserialize, Debug)]
pub struct FixedCapacity {
    pub state_of_charge: f64,
}

#[derive(Deserialize, Debug)]
pub struct FixedIrradiance {
    pub value: f64,
}

#[derive(Deserialize, Debug)]
pub struct VariableIrradiance {
    pub peak_value: f64,
}

#[derive(Deserialize, Debug)]
pub struct Exercises {
    pub fixed_distance: FixedDistance,
    pub fixed_capacity: FixedCapacity,
    pub fixed_irradiance: FixedIrradiance,
    pub variable_irradiance: VariableIrradiance,
}

#[derive(Deserialize, Debug)]
pub struct VerifyResponse {
    #[allow(unused)]
    pub code: String,
    #[allow(unused)]
    pub message: String,
}
