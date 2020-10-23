use super::errors::*;
use super::payload::login;
use super::payload::*;
use super::utils::is_authenticated::check;
use super::utils::start_time::chech_time;
use crate::database::actions;
use crate::database::pool::ConnectionPool;
use crate::user::User;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};

pub async fn login(
    id: Identity,
    json: web::Json<login::LoginRequest>,
    pool: web::Data<ConnectionPool>,
) -> ServiceResult<impl Responder> {
    chech_time()?;
    let conn = pool.get()?;
    debug!("{:?}", json.userID);
    let user = web::block(move || actions::find_user_by_userid(json.userID.trim(), &conn)).await?;
    debug!("{:?}", user);
    if let Some(user) = user {
        debug!("{}", user.userid);
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

pub async fn leaderoard(
    id: Identity,
    pool: web::Data<ConnectionPool>,
) -> ServiceResult<impl Responder> {
    use crate::payload::leaderboard::Leaderboard;
    chech_time()?;
    check(&id).await?;
    let conn = pool.get()?;
    let topTwenty = web::block(move || actions::get_top_twenty(&conn)).await?;
    let requestion_user = id.identity().unwrap();
    let conn = pool.get()?;
    let user = web::block(move || actions::find_user_by_userid(&requestion_user, &conn)).await?;
    let resp = Leaderboard {
        topTwenty,
        you: user.unwrap(),
    };
    Ok(HttpResponse::Ok().json(resp))
}

use challenges::challenges::*;
pub async fn check_response(
    id: Identity,
    json: web::Json<CheckResponseRequestActual>,
    pool: web::Data<ConnectionPool>,
) -> ServiceResult<impl Responder> {
    use crate::utils::duplicate::detect_duplicate;
    chech_time()?;
    check(&id).await?;
    let userid = id.identity().unwrap();
    let payload = json.into_inner();
    let verdict = check_answer(&payload)?;
    let resp = CheckResponseResponse {
        isCorrect: verdict.isCorrect,
    };
    let score = verdict.score;
    if verdict.isCorrect {
        let conn = pool.get()?;
        let user = web::block(move || actions::find_user_by_userid(&userid, &conn)).await?;
        if user.is_none() {
            return Err(ServiceError::AuthorizationRequired);
        } else {
            detect_duplicate(user.unwrap(), payload.id)?;
        }

        let userid = id.identity().unwrap();

        let conn = pool.get()?;
        web::block(move || actions::update_score(score, payload.id, &conn, &userid)).await?;
    }
    Ok(HttpResponse::Ok().json(resp))
}

pub async fn get_state(id: Identity) -> ServiceResult<impl Responder> {
    chech_time()?;
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

pub async fn get_questions(
    id: Identity,
    pool: web::Data<ConnectionPool>,
) -> ServiceResult<impl Responder> {
    chech_time()?;
    check(&id).await?;
    let requestion_user = id.identity().unwrap();
    let conn = pool.get()?;
    let insertable_user =
        web::block(move || actions::find_user_by_userid(&requestion_user, &conn)).await?;
    let user: User = insertable_user.unwrap().into();
    let progress = user.progress;
    let resp = challenges::challenges::get_challenges(&progress);
    Ok(HttpResponse::Ok().json(resp))
}
