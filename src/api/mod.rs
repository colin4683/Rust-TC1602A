use std::io::{Error, ErrorKind};
use reqwest;
use serde::de::DeserializeOwned; // Import necessary trait

pub struct API;

impl API {

    pub fn make_get<T: DeserializeOwned>(url: &str) -> Result<T, Error> {
        let client = reqwest::blocking::Client::new();
        let response = client.get(url)
            .send()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        if !response.status().is_success() {
            println!("{:?}", response);
            return Err(Error::new(ErrorKind::InvalidData, "Failed to get api data"));
        }
        println!("request success");
        let response_text = response.text().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let value = serde_json::from_str::<T>(&response_text).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(value)
    }

}