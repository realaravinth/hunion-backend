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
use crate::database::actions;
use crate::database::pool::ConnectionPool;

pub async fn login(
    id: Identity,
    json: web::Json<login::LoginRequest>,
    pool: web::Data<ConnectionPool>,
) -> ServiceResult<impl Responder> {
    chech_time()?;
    let conn = pool.get()?;
    let user = web::block(move || actions::find_user_by_userid(json.userID.trim(), &conn)).await?;
    if let Some(user) = user {
        let response = login::LoginResponse::new();
        id.remember(user.userid.to_owned()); // <- remember identity
        Ok(HttpResponse::Ok().json(response))
    } else {
        Err(ServiceError::AuthorizationRequired)
    }
}

pub async fn logout(id: Identity) -> ServiceResult<impl Responder> {
    check(&id).await?;
    id.forget();
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
    pool: web::Data<ConnectionPool>,
) -> ServiceResult<impl Responder> {
    chech_time()?;
    check(&id).await?;
    let isCorrect = check_answer(&json.into_inner())?;
    let resp = CheckResponseResponse { isCorrect };
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_state(id: Identity) -> ServiceResult<impl Responder> {
    //    chech_time()?;
    check(&id).await?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn register(
    pool: web::Data<ConnectionPool>,
    json: web::Json<register::RegisterRequestPayload>,
) -> ServiceResult<impl Responder> {
    let conn = pool.get()?;
    web::block(move || actions::insert_new_user(json.into_inner(), &conn)).await?;
    Ok(HttpResponse::Ok())
}

pub async fn get_questions(id: Identity) -> ServiceResult<impl Responder> {
    chech_time()?;
    check(&id).await?;
    use crate::user::Progress;
    let progress = [false; 7];
    let resp = challenges::challenges::get_challenges(&progress);
    Ok(HttpResponse::Ok().json(resp))
}
