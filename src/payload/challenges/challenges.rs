use serde::{Deserialize, Serialize};

use crate::user::Progress;

use crate::errors::*;

use super::*;

//#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
//pub struct GetChallengesResponse<'a> {
//    pub challenges: [Challenge<'a>; 7],
//}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct Challenge<'a> {
    pub id: u32,
    pub challenge_title: &'a str,
    pub challenge_body: &'a str,
    pub challenge_answer: Option<&'a str>,
    pub score: u32,
    pub has_answered: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct CheckResponseRequestActual {
    pub id: u32,
    pub user_answer: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct CheckResponseRequest<'a> {
    pub id: u32,
    pub user_answer: &'a str,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct CheckResponseResponse {
    pub is_correct: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct CheckResponseResponseBuilder {
    pub score: i32,
    pub is_correct: bool,
}

pub fn get_challenges(progress: &Progress) -> [&Challenge<'_>; 7] {
    [
        one::generate(&progress[0]),
        two::generate(&progress[1]),
        three::generate(&progress[2]),
        four::generate(&progress[3]),
        five::generate(&progress[4]),
        six::generate(&progress[5]),
        seven::generate(&progress[6]),
    ]
}

pub fn check_answer(
    payload: &CheckResponseRequestActual,
) -> ServiceResult<CheckResponseResponseBuilder> {
    match payload.id {
        1 => Ok(one::check(&payload.user_answer)),
        2 => Ok(two::check(&payload.user_answer)),
        3 => Ok(three::check(&payload.user_answer)),
        4 => Ok(four::check(&payload.user_answer)),
        5 => Ok(five::check(&payload.user_answer)),
        6 => Ok(six::check(&payload.user_answer)),
        7 => Ok(seven::check(&payload.user_answer)),
        _ => Err(ServiceError::InternalServerError),
    }
}
//            Challenge::generate_payload(&progress[0], challenges[0].clone()),
//            Challenge::generate_payload(&progress[1], challenges[1].clone()),
//            Challenge::generate_payload(&progress[2], challenges[2].clone()),
//            Challenge::generate_payload(&progress[3], challenges[3].clone()),
//            Challenge::generate_payload(&progress[4], challenges[4].clone()),
//            Challenge::generate_payload(&progress[5], challenges[5].clone()),
//            Challenge::generate_payload(&progress[6], challenges[6].clone()),

//impl Challenge {
//    pub fn new(
//        id: u32,
//        challenge_title: String,
//        challenge_body: String,
//        challenge_answer: String,
//        score: u32,
//    ) -> Self {
//        Challenge {
//            id,
//            challenge_title,
//            challenge_body,
//            challenge_answer: Some(challenge_answer),
//            score,
//        }
//    }
//
//    pub fn generate_payload(has_answered: bool, challenge: Challenge) -> Self {
//        if has_answered {
//            challenge
//        } else {
//            Challenge {
//                challenge_answer: None,
//                ..challenge
//            }
//        }
//    }
//}
//
