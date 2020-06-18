#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate reqwest;

use rocket_contrib::json::{Json};
use serde::{Deserialize};
use reqwest::ClientBuilder;

mod types;
mod description;

#[derive(Deserialize)]
struct ContentsJson {
    translated: String
}

#[derive(Deserialize)]
struct TransationResponse {
    contents: ContentsJson
}

fn translate(description: &str) -> types::ServiceResult<String> {
    let body = json!({
        "text": description
    });
    let api_url = "https://api.funtranslations.com/translate/shakespeare.json";
    
    let client = match ClientBuilder::new().build() {
        Ok(client) => client,
        Err(_err) => return Err(types::PokemonError {
            status: String::from("error"),
            reason: String::from("Not possible to establish connection with translation service")
        })
    };
    
    let mut response = match client.post(api_url).json(&body).send() {
        Ok(response) => response,
        Err(_err) => return Err(types::PokemonError {
            status: String::from("error"),
            reason: String::from("Not possible to obtain translation")
        })
    };

    let response_json: TransationResponse = match response.json() {
        Ok(response_json) => response_json,
        Err(_err) => return Err(types::PokemonError {
            status: String::from("error"),
            reason: String::from("Translation does not contain valid JSON")
        })
    };

    Ok(response_json.contents.translated)
}

#[get("/<name>", format = "json")]
fn get_by(name: String) -> types::ApiResult<types::PokemonDescription> {
    let description = match description::fetch_description(&name) {
        Ok(description) => description.to_owned(),
        Err(err) => return Err(Json(err)),
    };
    let translation = match translate(&description) {
        Ok(translation) => translation,
        Err(err) => return Err(Json(err)),
    };
    Ok(Json(types::PokemonDescription {
        name,
        description: translation
    }))
}

#[catch(404)]
fn not_found() -> Json<types::PokemonError> {
    Json(types::PokemonError {
        status: String::from("error"),
        reason: String::from("Resource was not found")
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/pokemon", routes![get_by])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
