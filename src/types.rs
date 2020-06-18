use rocket_contrib::json::{Json};
use serde::{Serialize};

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

pub type ServiceResult<T> = Result<T, PokemonError>;

pub type ApiResult<T> = Result<Json<T>, Json<PokemonError>>;