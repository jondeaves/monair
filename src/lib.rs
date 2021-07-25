use serde::Deserialize;
use serde_json::{Result, Value};
use std::any::Any;

#[derive(Deserialize, Debug)]
pub struct MonairConfig {
    pub api_key: String,
    pub lat: f64,
    pub long: f64,
}

#[derive(Deserialize, Debug)]
pub struct AirQualityResponse {
    pub location_name: String,
    pub pm10: f32,
}

#[tokio::main(flavor = "current_thread")]
pub async fn run(config: MonairConfig) -> Result<()> {
    let request_url = format!(
        "https://api.waqi.info/feed/geo:{lat};{long}/?token={api_key}",
        lat = config.lat,
        long = config.long,
        api_key = config.api_key
    );

    let resp = reqwest::get(&request_url).await.unwrap_or_else(|err| {
        panic!("Could not fetch data: {:?}", err);
    });

    let resp_json = resp.json().await.unwrap_or_else(|err| {
        panic!("Could not get JSON from response: {:?}", err);
    });

    let payload: Value = serde_json::from_str(resp_json)?;

    let air_quality_response = AirQualityResponse {
        location_name: payload["data"]["city"]["name"].to_string(),
        pm10: match payload["data"]["iaqi"]["pm10"].to_string().parse::<f32>() {
            Ok(value) => value,
            Err(_) => 0.0,
        },
    };

    println!("{:#?}", air_quality_response);
    println!("{}", air_quality_response.location_name);

    Ok(())
}
