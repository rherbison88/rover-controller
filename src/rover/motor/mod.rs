use serde::Deserialize;

mod wheel;

#[derive(Deserialize, Debug)]
pub struct Motor {
    name: String,
    kv_rating: f64,
    current_rating: f64,
    wheel: wheel::Wheel,
}
