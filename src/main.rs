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

mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    let addr = env::var("PORT").expect("Please set PORT to the port that you wish to listen to");
    let secret =
        env::var("SECRET").expect("Please set SECRET to the port that you wish to listen to");
    let domain =
        env::var("DOMAIN").expect("Please set DOMAIN to the port that you wish to listen to");

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(
                CookieSession::signed(&secret.as_bytes())
                    .domain(&domain)
                    .name("shuttlecraft-session")
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
                    .max_age(20)
                    .domain(&domain)
                    .same_site(SameSite::Lax)
                    .secure(true),
            ))
            .service(Files::new("/", "/var/www/amnesia-client/static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
