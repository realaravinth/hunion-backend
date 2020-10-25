#![warn(rust_2018_idioms, elided_lifetimes_in_paths)]
use pretty_env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate diesel;
use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
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

use crate::handlers::*;
use database::pool::get_connection_pool;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let _ = env::var("ROOT").expect("Please set ROOT to the port that you wish to listen to");
    let _ = env::var("START_TIME")
        .expect("Please set START_TIME to the port that you wish to listen to");
    let port = env::var("PORT").expect("Please set PORT to the port that you wish to listen to");
    let secret = env::var("SECRET").expect("Please set SECRET");
    let domain =
        env::var("DOMAIN").expect("Please set DOMAIN to the port that you wish to listen to");
    let database_url = env::var("DATABASE_URL")
        .expect("Please set DATABASE_URL to the port that you wish to listen to");
    let static_files = env::var("static_files")
        .expect("Please set static_files to the port that you wish to listen to");

    let database_connection_pool = get_connection_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .data(database_connection_pool.clone())
            .wrap(Compress::default())
            //    .wrap(
            //        Cors::default()
            //            .allowed_origin("http://localhost:3000")
            //            .allowed_methods(vec!["GET", "POST"])
            //            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //            .allowed_header(header::CONTENT_TYPE)
            //            .supports_credentials()
            //            .max_age(3600),
            //    )
            .wrap(
                CookieSession::signed(&secret.as_bytes())
                    .domain(&domain)
                    .name("BfkhiQuM&yBgq!9SMrLH%*4vePY$LMV!2u4B!XewAhZXsC%nWn^pUcjx@exbR3N9")
                    .path("/")
                    .secure(true),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&secret.as_bytes())
                    .name("Authorization")
                    .max_age(36000)
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
            .service(Files::new("/", &static_files).index_file("index.html"))
    })
    .bind(format!("0.0.0.0:{}", &port))?
    .run()
    .await
}
