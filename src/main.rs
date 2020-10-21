#![warn(rust_2018_idioms)]
use pretty_env_logger;
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpServer, Result,
};
use dotenv::dotenv;
use std::env;

mod database;
mod errors;
mod handlers;
mod models;
mod payload;
mod utils;

use crate::handlers::*;
use crate::payload::challenges::*;
use challenges::Challenge;

lazy_static! {
    pub static ref ROOT: String =
        env::var("ROOT").expect("Please set ROOT to the port that you wish to listen to");
    pub static ref START_TIME: StringNumer = get_vars("START_TIME");
    pub static ref END_TIME: StringNumer = get_vars("END_TIME");
    pub static ref START_TIME_STRING: String = env::var("START_TIME")
        .expect("Please set START_TIME to the port that you wish to listen to");
}

fn get_vars(var_name: &str) -> StringNumer {
    let value = env::var(var_name).expect(&format!("Please set {}", var_name));
    StringNumer::Alpha(value.to_owned())
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum StringNumer {
    Numeric(u64),
    Alpha(String),
}

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

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
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
                    .same_site(SameSite::Strict)
                    .secure(true),
            ))
            .route("/api/login", web::post().to(login))
            .route("/api/logout", web::get().to(logout))
            .route("/api/leaderboard", web::get().to(leaderoard))
            .route("/api/get-state", web::get().to(get_state))
            .route("/api/check-response", web::post().to(check_response))
            .route("/api/get-challenges", web::get().to(get_questions))
            .route("/api/register", web::post().to(register))
            .default_service(Files::new("/[^a-zA-Z0-9]", &STATIC).index_file("index.html"))
    })
    .keep_alive(0)
    .bind(format!("0.0.0.0:{}", &port))?
    .run()
    .await
}
