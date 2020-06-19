use rocket::{self, local::Client, http::Status};

#[test]
fn get_by() {
    let rocket = rocket::ignite()
        .mount("/pokemon", routes![super::get_by])
        .register(catchers![super::not_found])
        .manage(super::init_client().unwrap());
    let client = Client::new(rocket).unwrap();
    let response = client.get("/pokemon/pikachu").dispatch();
    assert_eq!(response.status(), Status::Ok);
}

#[test]
fn url_not_found() {
    let rocket = rocket::ignite()
        .mount("/pokemon", routes![super::get_by])
        .register(catchers![super::not_found])
        .manage(super::init_client().unwrap());
    let client = Client::new(rocket).unwrap();
    let response = client.get("/abc/pikachu").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn pokemon_not_found() {
    let rocket = rocket::ignite()
        .mount("/pokemon", routes![super::get_by])
        .register(catchers![super::not_found])
        .manage(super::init_client().unwrap());
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/pokemon/abc").dispatch();
    let body_error = "{\"status\":\"error\",\"reason\":\"Pokemon was not found\"}";
    assert_eq!(response.body_string(), Some(body_error.into()));
}
