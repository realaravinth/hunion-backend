use super::challenges::Challenge;
pub static CHALLENGE_ANSWER: &'static str = "zlZ6QPh97sg16ds856RK0DIK1waNJwy";
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
    let challenge_body = r#" <p>Hack this page to retrieve the flag for this challenge</p> "#;
    let challenge_title = "hack_the_gibson";
    let ans = "zlZ6QPh97sg16ds856RK0DIK1waNJwy";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        id: 6,
        challenge_title,
        challenge_body,
        challenge_answer: answer,
        score: 20,

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
