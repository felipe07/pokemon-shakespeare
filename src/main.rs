use std::collections::HashMap;

use actix_http::ResponseBuilder;
use actix_web::{
    client::Client, error, get, http::header, http::StatusCode, web, App, HttpResponse,
    HttpServer, Result,
};
use failure::Fail;
use pokerust::{FromName, PokemonSpecies};
use serde::{Deserialize, Serialize};
use bincode::{deserialize};

#[derive(Serialize, Fail, Debug)]
enum PokemonServiceError {
    #[fail(display = "Pokemon not found")]
    PokemonNotFound,
    #[fail(display = "Internal error")]
    InternalError,
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

#[derive(Serialize)]
struct PokemonDescription {
    name: String,
    description: String,
}

#[derive(Deserialize)]
struct TranslationSuccess {
    total: u32,
}

#[derive(Deserialize)]
struct TranslationContent {
    translated: String,
    text: String,
    translation: String,
}

#[derive(Deserialize)]
struct PokemonDescriptionTranslation {
    success: TranslationSuccess,
    contents: TranslationContent,
}

async fn get_description(name: String) -> DescriptionResult<PokemonDescription> {
    let poke = match PokemonSpecies::from_name(&name) {
        Ok(poke) => poke,
        Err(_err) => return Err(PokemonServiceError::PokemonNotFound),
    };

    let flavor_texts = poke.flavor_text_entries;
    let mut iter_flavor_texts = flavor_texts.iter();
    let flavor_text_en = iter_flavor_texts.find(|&flavor_text| {
        flavor_text.language.name == "en"
            && flavor_text.version.as_ref().unwrap().name == "omega-ruby"
    });
    let description_en = flavor_text_en
        .unwrap()
        .flavor_text
        .to_string()
        .replace("\n", " ");
    //let translation: PokemonDescriptionTranslation = translate(description_en).await?;
    translate(&description_en).await;
    Ok(PokemonDescription {
        name,
        description: description_en
    })
}

//async fn translate(description_en: String) -> DescriptionResult<PokemonDescriptionTranslation> {
async fn translate(description_en: &str) {
    let request_url = "https://api.funtranslations.com/translate/shakespeare.json";

    let mut params = HashMap::new();
    params.insert("text", description_en);

    let client = Client::default();
    client::post(url)
        .send_form(&params)
        .from_err()
        .and_then(|res: ClientResponse| {
            println!("{:?}", res);
            //Ok(HttpResponse::Ok().body("inside future"))
        })
        .responder()

    /*let content = match translationCall {
        Ok(content) => &content.body().await.unwrap(),
        Err(_err) => return Err(PokemonServiceError::InternalError),
    };*/

    // let d = deserialize(&content)?;
    //println!("{:?}", translationCall);
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
    HttpServer::new(|| App::new().service(description))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
