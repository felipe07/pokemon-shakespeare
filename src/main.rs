#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::{Json};

mod types;
mod description;
mod translator;

#[get("/<name>", format = "json")]
fn get_by(name: String) -> types::ApiResult<types::PokemonDescription> {
    let description = match description::fetch_description(&name) {
        Ok(description) => description.to_owned(),
        Err(err) => return Err(Json(err)),
    };
    let translation = match translator::translate(&description) {
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
}

fn main() {
    rocket().launch();
}
