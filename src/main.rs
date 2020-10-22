#![warn(rust_2018_idioms)]
use pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
use actix_cors::Cors;
use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    http::header,
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

use dotenv::dotenv;
use std::env;

mod database;
mod errors;
mod handlers;
mod models;
mod payload;
mod schema;
mod user;
mod utils;

use crate::errors::*;
use crate::handlers::*;
use crate::payload::challenges::*;
use challenges::Challenge;
use database::pool::get_connection_pool;
use std::fs::read_to_string;

lazy_static! {
    pub static ref ROOT: String =
        env::var("ROOT").expect("Please set ROOT to the port that you wish to listen to");
    pub static ref START_TIME: u64 = env::var("START_TIME")
        .expect("Please set START_TIME to the port that you wish to listen to")
        .parse()
        .expect("please enter valid time");
    pub static ref END_TIME: u64 = env::var("END_TIME")
        .expect("Please set END_TIME to the port that you wish to listen to")
        .parse()
        .expect("please enter valid time");
    pub static ref START_TIME_STRING: String = env::var("START_TIME")
        .expect("Please set START_TIME to the port that you wish to listen to");
}

//pub async fn index() -> ServiceResult<impl Responder> {
//    let index = read_to_string(format!("{}/index.html", env::var("STATIC").unwrap())).unwrap();
//
//    Ok(HttpResponse::Ok().content_type("text/html").body(index))
//}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let _ = env::var("ROOT").expect("Please set ROOT to the port that you wish to listen to");
    let _ = env::var("START_TIME")
        .expect("Please set START_TIME to the port that you wish to listen to");
    //    let _ = env::var("DURATION_IN_SECONDS").expect("Please set DURATION_IN_SECONDS");
    let port = env::var("PORT").expect("Please set PORT to the port that you wish to listen to");
    let secret = env::var("SECRET").expect("Please set SECRET");
    let domain =
        env::var("DOMAIN").expect("Please set DOMAIN to the port that you wish to listen to");
    let DATABASE_URL = env::var("DATABASE_URL")
        .expect("Please set DATABASE_URL to the port that you wish to listen to");
    let STATIC =
        env::var("STATIC").expect("Please set STATIC to the port that you wish to listen to");

    let database_connection_pool = get_connection_pool(&DATABASE_URL);

    let index = read_to_string(format!("{}/index.html", &STATIC)).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(database_connection_pool.clone())
            .wrap(Compress::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(
                CookieSession::signed(&secret.as_bytes())
                    .domain(&domain)
                    .name(&domain)
                    .path("/")
                    .secure(true),
            )
            .wrap(
                CookieSession::signed(&secret.as_bytes())
                    .domain(&domain)
                    .name("on")
                    .path("/")
                    .secure(false),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&secret.as_bytes())
                    .name("Authorization")
                    .max_age(3600)
                    .domain(&domain)
                    .same_site(SameSite::Lax)
                    .secure(false),
            ))
            .wrap(Logger::default())
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::get().to(logout))
            .route("/api/leaderboard", web::get().to(leaderoard))
            .route("/api/get-state", web::get().to(get_state))
            .route("/api/check-response", web::post().to(check_response))
            .route("/api/get-challenges", web::get().to(get_questions))
            .route("/api/register", web::post().to(register))
            .service(Files::new("/", &STATIC).index_file("index.html"))
    })
    .bind(format!("0.0.0.0:{}", &port))?
    .run()
    .await
}
