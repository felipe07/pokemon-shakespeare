#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate reqwest;

use rocket_contrib::json::{Json};
use serde::{Serialize, Deserialize};
use pokerust::{FromName, PokemonSpecies};
use reqwest::ClientBuilder;

#[derive(Serialize)]
struct PokemonDescription {
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
struct PokemonError {
    status: String,
    reason: String,
}

#[derive(Deserialize)]
struct ContentsJson {
    translated: String
}

#[derive(Deserialize)]
struct TransationResponse {
    contents: ContentsJson
}

type ServiceResult<T> = Result<T, PokemonError>;
type ApiResult<T> = Result<Json<T>, Json<PokemonError>>;

fn fetch_description(name: &str) -> ServiceResult<String> {
    let pokemon_name = match PokemonSpecies::from_name(&name) {
        Ok(pokemon_name) => pokemon_name,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Pokemon was not found")
        }),
    };

    let flavor_texts = pokemon_name.flavor_text_entries;
    let mut iter_flavor_texts = flavor_texts.iter();
    let flavor_text_en = iter_flavor_texts.find(|&flavor_text| {
        flavor_text.language.name == "en"
            && flavor_text.version.as_ref().unwrap().name == "omega-ruby"
    });
    let description_en = flavor_text_en
        .unwrap()
        .flavor_text
        .to_string()
        .replace("\n", "");
    
    Ok(description_en)
}

fn translate(description: &str) -> ServiceResult<String> {
    let body = json!({
        "text": description
    });
    let api_url = "https://api.funtranslations.com/translate/shakespeare.json";
    
    let client = match ClientBuilder::new().build() {
        Ok(client) => client,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Not possible to establish connection with translation service")
        })
    };
    
    let mut response = match client.post(api_url).json(&body).send() {
        Ok(response) => response,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Not possible to obtain translation")
        })
    };

    let response_json: TransationResponse = match response.json() {
        Ok(response_json) => response_json,
        Err(_err) => return Err(PokemonError {
            status: String::from("error"),
            reason: String::from("Translation does not contain valid JSON")
        })
    };

    Ok(response_json.contents.translated)
}

#[get("/<name>", format = "json")]
fn get_by(name: String) -> ApiResult<PokemonDescription> {
    let description = match fetch_description(&name) {
        Ok(description) => description,
        Err(err) => return Err(Json(err)),
    };
    let translation = match translate(&description) {
        Ok(translation) => translation,
        Err(err) => return Err(Json(err)),
    };
    Ok(Json(PokemonDescription {
        name,
        description: translation
    }))
}

#[catch(404)]
fn not_found() -> Json<PokemonError> {
    Json(PokemonError {
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
