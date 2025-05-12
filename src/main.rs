use std::error::Error;
use serde::Deserialize;

mod exercises;
mod rover;

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

    let client = reqwest::Client::new();

    let health: Health = client.get("http://localhost:8080/health")
        .send()
        .await?
        .json::<HealthResp>()
        .await?
        .into();

    println!("{health:#?}\n\n");

    let resp = client.get("http://localhost:8080/exercises")
        .send()
        .await?
        .json::<exercises::Exercises>()
        .await?;

    println!("{resp:#?}\n\n");

    let resp = client.get("http://localhost:8080/rover/config")
        .send()
        .await?
        .json::<rover::Rover>()
        .await?;

    println!("{resp:#?}");

    Ok(())
}
