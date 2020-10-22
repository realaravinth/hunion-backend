use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

use super::errors::*;
use super::payload::login;
use super::payload::*;
use super::utils::is_authenticated::check;
use super::utils::start_time::chech_time;

pub async fn login(
    id: Identity,
    json: web::Json<login::LoginRequest>,
) -> ServiceResult<impl Responder> {
    chech_time()?;
    debug!("{:?}", &json.userID);
    if json.userID == "a" {
        let response = login::LoginResponse::new();
        id.remember("User1".to_owned()); // <- remember identity
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(ServiceError::AuthorizationRequired)
    }
}

pub async fn logout(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    id.forget(); // <- remove identity
    Ok(HttpResponse::Ok().finish())
}

pub async fn leaderoard(id: Identity) -> ServiceResult<impl Responder> {
    chech_time()?;
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}

use challenges::challenges::*;
pub async fn check_response(
    id: Identity,
    json: web::Json<CheckResponseRequestActual>,
) -> ServiceResult<impl Responder> {
    //chech_time()?;
    //check(&id).await?;
    let isCorrect = check_answer(&json.into_inner())?;
    let resp = CheckResponseResponse { isCorrect };
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_state(id: Identity) -> ServiceResult<impl Responder> {
    //    chech_time()?;
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn register() -> ServiceResult<impl Responder> {
    use crate::ROOT;
    //TODO    do root shit
    Ok(HttpResponse::Ok().finish())
}

pub async fn get_questions(id: Identity) -> ServiceResult<impl Responder> {
    //chech_time()?;
    //check(&id).await?;
    use crate::models::Progress;
    let progress = [false; 7];
    let resp = challenges::challenges::get_challenges(&progress);
    Ok(HttpResponse::Ok().json(resp))
}
