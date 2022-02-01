use std::fs;

use actix_web::{get, post, web, HttpResponse, Responder};

use crate::country::Country;

#[get("/")]
async fn home() -> impl Responder {
    // read from file, our "database"
    let mut country = Country::default();
    country.name = String::from("Hello");
    country.capital = String::from("World!");
    HttpResponse::Ok().json(vec![country])
}

#[get("/countries")]
async fn find_all_countries() -> impl Responder {
    // read from file, our "database"
    let data = fs::read_to_string("data.txt").unwrap();
    let countries: Vec<Country> = serde_json::from_str(&data).unwrap();
    HttpResponse::Ok().json(countries)
}

#[get("/country/{id}")]
async fn find_country(web::Path(country_id): web::Path<i32>) -> impl Responder {
    let data = fs::read_to_string("data.txt").unwrap();
    let countries: Vec<Country> = serde_json::from_str(&data).unwrap();
    HttpResponse::Ok().json(&countries.get(country_id as usize))
}

#[post("/country")]
async fn create_country(country: web::Json<Country>) -> impl Responder {
    let data = fs::read_to_string("data.txt").unwrap();
    let mut countries: Vec<Country> = serde_json::from_str(&data).unwrap();

    let country2 = Country {
        id: countries.len() as i32,
        name: country.name.clone(),
        capital: country.capital.clone(),
        area: country.area.clone(),
    };

    countries.push(country2);

    let s = serde_json::to_string(&countries).unwrap();
    fs::write("data.txt", s).unwrap();

    HttpResponse::Ok().body(format!("Country created: {}\n", country.name))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(home);
    cfg.service(find_all_countries);
    cfg.service(find_country);
    cfg.service(create_country);
}
