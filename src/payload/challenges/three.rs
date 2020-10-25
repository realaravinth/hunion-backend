use super::challenges::Challenge;

pub static CHALLENGE_ANSWER: &'static str = "E5w&Wwv1jRVRuHT^yf8&%";
pub static UNANSWERED: &'static Challenge<'static> = &init(&false);

pub static ANSWERED: &'static Challenge<'static> = &init(&true);

pub fn generate<'a>(has_answered: &'a bool) -> &Challenge<'_> {
    if *has_answered {
        ANSWERED
    } else {
        UNANSWERED
    }
}

pub const fn init<'a>(has_answered: &'a bool) -> Challenge<'_> {
    let ans = "E5w&Wwv1jRVRuHT^yf8&%";
    let challenge_body = r#" <p> What does 'RTV3Jld3djFqUlZSdUhUXnlmOCYl' mean? I think it has something to do with a binary-to-text encoding that's popular on the World Wide Web </p> "#;
    let challenge_title = ".... --- .-- -.. -.--";
    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        challenge_title,
        challenge_body,
        challenge_answer: answer,
        score: 20,
        id: 3,

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
