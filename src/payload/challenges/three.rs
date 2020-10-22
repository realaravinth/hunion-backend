use super::challenges::Challenge;

pub static challengeAnswer: &'static str = "E5w&Wwv1jRVRuHT^yf8&%";
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
    let ans = "E5w&Wwv1jRVRuHT^yf8&%";
    let challengeBody = r#" <p> What does 'RTV3Jld3djFqUlZSdUhUXnlmOCYl' mean? I think it has something to do with a binary-to-text encoding that's popular on the World Wide Web </p> "#;
    let challengeTitle = ".... --- .-- -.. -.--";
    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        challengeTitle,
        challengeBody,
        challengeAnswer: answer,
        score: 20,
        id: 3,

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
