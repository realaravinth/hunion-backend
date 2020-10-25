use super::challenges::Challenge;

pub static CHALLENGE_ANSWER: &'static str = "7641648545";

pub static UNANSWERED: &'static Challenge<'static> = &init(&false);

pub static ANSWERED: &'static Challenge<'static> = &init(&true);

pub fn generate<'a>(has_answered: &'a bool) -> &Challenge<'_> {
    if *has_answered {
        ANSWERED
    } else {
        UNANSWERED
    }
}

pub const fn init(has_answered: &bool) -> Challenge<'_> {
    let challenge_body = r#" <p id='Obi-Wan-Kenobi'> To a computer, everything is either a 1 or a 0. Even if you feed it an image of pizzas, it'll still be a bunch of 0s and 1s. This way of representing things is called base-2 numeral system or binary numeral system. The flag for this challenge is a 10-digit number in the decimal number system(ranges from 0 to 9). <br /> <br />We have given your computer special instructions to hide it away from you. But you know how computers are,they are high maintenance. So maybe if you ask it nicely, it might tell you what the flag is! </p> <br /> <style> .hidden { visibility: hidden; } .show { visibility: visible; } </style> "#;
    let challenge_title = "be3p_b00p_be3p_b00p";
    let ans = "7641648545";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        id: 1,
        challenge_title,
        challenge_body,
        challenge_answer: answer,
        score: 10,
        has_answered: *has_answered,
    }
}

use super::challenges::CheckResponseResponseBuilder;
pub fn check(user_answer: &str) -> CheckResponseResponseBuilder {
    if user_answer.trim() == CHALLENGE_ANSWER {
        CheckResponseResponseBuilder {
            is_correct: true,
            score: UNANSWERED.score as i32,
        }
    } else {
        CheckResponseResponseBuilder {
            is_correct: false,
            score: UNANSWERED.score as i32,
        }
    }
}
