#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
	
#[cfg(test)] mod tests;

use rocket_contrib::json::{Json};
use rocket::State;
use reqwest::{Client, ClientBuilder};

mod types;
mod description;
mod translator;

#[get("/<name>", format = "json")]
fn get_by(name: String, client: State<Client>) -> types::ApiResult<types::PokemonDescription> {
    let description = match description::fetch_description(&name) {
        Ok(description) => description.to_owned(),
        Err(err) => return Err(Json(err)),
    };
    let translation = match translator::translate(&description, client.inner()) {
        Ok(translation) => translation.to_owned(),
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
        .manage(init_client().unwrap())
}

fn init_client() -> Result<Client, types::PokemonError>{
    match ClientBuilder::new().build() {
        Ok(client) => return Ok(client),
        Err(_err) => return Err(types::PokemonError {
            status: String::from("error"),
            reason: String::from("Not possible to initialize HTTP client")
        })
    };
}

fn main() {
    rocket().launch();
}
