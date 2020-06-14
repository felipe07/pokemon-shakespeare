use actix_http::ResponseBuilder;
use actix_web::{get, web, App, error, http::header, http::StatusCode, HttpResponse, HttpServer, Result};
use failure::Fail;
use serde::{Deserialize, Serialize};
use pokerust::{PokemonSpecies, FromName};

#[derive(Serialize)]
#[derive(Fail, Debug)]
enum PokemonServiceError {
    #[fail(display = "bad request")]
    PokemonNotFound,
    #[fail(display = "internal error")]
    InternalError
}

impl error::ResponseError for PokemonServiceError {
    fn error_response(&self) -> HttpResponse {
        ResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            PokemonServiceError::PokemonNotFound => StatusCode::NOT_FOUND,
            PokemonServiceError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

type DescriptionResult<T> = Result<T, PokemonServiceError>;

#[derive(Serialize, Deserialize)]
struct PokemonDescription {
    name: String,
    description: String
}

async fn get_description(name: String) -> DescriptionResult<PokemonDescription> {
    let poke = match PokemonSpecies::from_name(&name) {
        Ok(poke) => poke,
        Err(_err) => return Err(PokemonServiceError::PokemonNotFound),
    };

    let flavor_texts = poke.flavor_text_entries;
    let mut iter_flavor_texts = flavor_texts.iter();
    let description_en = iter_flavor_texts.find(
        |&flavor_text| 
            flavor_text.language.name == "en" && 
            flavor_text.version.as_ref().unwrap().name == "omega-ruby");
    
    Ok(PokemonDescription {
        name,
        description: description_en.unwrap().flavor_text.to_string()
    })
}

#[get("/pokemon/{name}")]
async fn description(name: web::Path<String>) -> Result<HttpResponse, PokemonServiceError> {
    let result = get_description(name.to_string()).await;
    match result {
        Ok(desc) => Ok(HttpResponse::Ok().json(desc)),
        Err(err) => Err(err),
    } 
    
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(description)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}