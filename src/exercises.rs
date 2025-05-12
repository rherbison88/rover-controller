use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct FixedDistance {
    value: f64
}

#[derive(Deserialize, Debug)]
struct FixedCapacity {
    state_of_charge: f64
}

#[derive(Deserialize, Debug)]
struct FixedIrradiance {
    value: f64
}

#[derive(Deserialize, Debug)]
struct VariableIrradiance {
    peak_value: f64
}

#[derive(Deserialize, Debug)]
pub struct Exercises {
    fixed_distance: FixedDistance,
    fixed_capacity: FixedCapacity,
    fixed_irradiance: FixedIrradiance,
    variable_irradiance: VariableIrradiance,
}

