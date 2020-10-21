use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

use super::errors::*;
use super::utils::is_authenticated::check;

pub async fn login(id: Identity) -> ServiceResult<impl Responder> {
    id.remember("User1".to_owned()); // <- remember identity
    Ok(HttpResponse::Ok().finish())
}

pub async fn logout(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    id.forget(); // <- remove identity
    Ok(HttpResponse::Ok().finish())
}

pub async fn leaderoard(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn check_response(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_state(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_questions(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}
