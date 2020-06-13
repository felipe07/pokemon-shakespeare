use actix_web::{web, Result, App, HttpResponse, HttpServer, Responder};
use actix_web::get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PokemonDescription {
    name: String,
    description: String
}

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn get_description(name: String) -> PokemonDescription {
    PokemonDescription {
        name,
        description: "Mocked pokemon's description".to_string()
    }
}

#[get("/pokemon/{name}")]
async fn description(name: web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        get_description(name.to_string()).await))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(description)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}