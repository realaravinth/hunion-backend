use serde::{Deserialize, Serialize};

use crate::models::Progress;
use crate::START_TIME_STRING;

use super::*;

//#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
//pub struct GetChallengesResponse<'a> {
//    pub challenges: [Challenge<'a>; 7],
//}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct Challenge<'a> {
    pub id: u32,
    pub challengeTitle: &'a str,
    pub challengeBody: &'a str,
    pub challengeAnswer: Option<&'a str>,
    pub score: u32,
}

pub fn reducer(progress: &Progress) -> [Challenge; 7] {
    [
        one::generate(&progress[0]),
        two::generate(&progress[1]),
        three::generate(&progress[2]),
        four::generate(&progress[3]),
        five::generate(&progress[4]),
        six::generate(&progress[5]),
        seven::generate(&progress[6]),
    ]
    .to_owned()
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
//        challengeTitle: String,
//        challengeBody: String,
//        challengeAnswer: String,
//        score: u32,
//    ) -> Self {
//        Challenge {
//            id,
//            challengeTitle,
//            challengeBody,
//            challengeAnswer: Some(challengeAnswer),
//            score,
//        }
//    }
//
//    pub fn generate_payload(has_answered: bool, challenge: Challenge) -> Self {
//        if has_answered {
//            challenge
//        } else {
//            Challenge {
//                challengeAnswer: None,
//                ..challenge
//            }
//        }
//    }
//}
//
