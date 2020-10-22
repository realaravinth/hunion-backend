use super::challenges::Challenge;
pub static challengeAnswer: &'static str = "zlZ6QPh97sg16ds856RK0DIK1waNJwy";
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
    let challengeBody = r#" <p>Hack this page to retrieve the flag for this challenge</p> "#;
    let challengeTitle = "hack_the_gibson";
    let ans = "zlZ6QPh97sg16ds856RK0DIK1waNJwy";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        id: 6,
        challengeTitle,
        challengeBody,
        challengeAnswer: answer,
        score: 20,

        hasAnswered: *has_answered,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer.trim() == challengeAnswer {
        true
    } else {
        false
    }
}
