#![warn(rust_2018_idioms)]
use pretty_env_logger;
#[macro_use]
extern crate log;

use actix_files::Files;
use actix_http::cookie::SameSite;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpServer, Result,
};
use std::env;
use std::path::PathBuf;

mod database;
mod errors;
mod handlers;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let port = env::var("PORT").expect("Please set PORT to the port that you wish to listen to");
    let secret =
        env::var("SECRET").expect("Please set SECRET to the port that you wish to listen to");
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
                    .name("hunion-session")
                    .path("/")
                    .secure(false),
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
            .service(Files::new("/", &STATIC).index_file("index.html"))
    })
    .bind(format!("0.0.0.0:{}", &port))?
    .run()
    .await
}
