use rocket_contrib::json::{Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct PokemonDescription {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct PokemonError {
    pub status: String,
    pub reason: String,
}

#[derive(Deserialize)]
pub struct Contents {
    pub translated: String
}

#[derive(Deserialize)]
pub struct Transation {
    pub contents: Contents
}

pub type ServiceResult<T> = Result<T, PokemonError>;

pub type ApiResult<T> = Result<Json<T>, Json<PokemonError>>;