use super::challenges::Challenge;

pub static challengeAnswer: &'static str = "7641648545";

pub static UNANSWERED: &'static Challenge = &init(&false);

pub static ANSWERED: &'static Challenge = &init(&true);

pub fn generate<'a>(has_answered: &'a bool) -> &Challenge {
    if *has_answered {
        ANSWERED
    } else {
        UNANSWERED
    }
}

pub const fn init<'a>(has_answered: &'a bool) -> Challenge {
    let challengeBody = r#" <p id='Obi-Wan-Kenobi'> To a computer, everything is either a 1 or a 0. Even if you feed it an image of pizzas, it'll still be a bunch of 0s and 1s. This way of representing things is called base-2 numeral system or binary numeral system. The flag for this challenge is a 10-digit number in the decimal number system(ranges from 0 to 9). <br /> <br />We have given your computer special instructions to hide it away from you. But you know how computers are,they are high maintenance. So maybe if you ask it nicely, it might tell you what the flag is! </p> <br /> <style> .hidden { visibility: hidden; } .show { visibility: visible; } </style> "#;
    let challengeTitle = "be3p_b00p_be3p_b00p";
    let ans = "7641648545";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        id: 1,
        challengeTitle,
        challengeBody,
        challengeAnswer: answer,
        score: 10,
        hasAnswered: *has_answered,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer == challengeAnswer {
        true
    } else {
        false
    }
}
