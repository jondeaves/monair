use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct MonairConfig {
    pub api_key: String,
    pub lat: f32,
    pub long: f32,
}

pub fn run(config: MonairConfig) -> Result<(), Box<dyn Error>> {
    println!("{:?}", config);

    Ok(())
}
