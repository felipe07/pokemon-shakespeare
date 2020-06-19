extern crate reqwest;

use reqwest::Client;
use crate::types::{ServiceResult, PokemonError, Transation};

const API_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

pub fn translate(description: &str, client: &Client) -> ServiceResult<String> {
    let body = json!({
        "text": description
    });
    
    let mut response = match client.post(API_URL).json(&body).send() {
        Ok(response) => response,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Not possible to obtain translation")
        })
    };

    let response_json: Transation = match response.json() {
        Ok(response_json) => response_json,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Translation does not contain valid JSON")
        })
    };

    Ok(response_json.contents.translated)
}