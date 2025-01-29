use std::io::{Error, ErrorKind};
use reqwest;
use serde::{Deserialize, Serialize};


pub struct API {
    url: String,
}

impl API {

    pub fn make_get<T>(url: String) -> Result<T, Error> {
        // get to the endpoint
        let client = reqwest::blocking::Client::new();
        let response = client.get(&url)
            .send()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        if !response.status().is_success() {
            return Err(Error::new(ErrorKind::InvalidData, "Failed to get api data"));
        }
        let response_text = response.text().map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        let value = serde_json::from_str::<T>(&response_text).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
        Ok(value)
    }

}