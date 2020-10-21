use super::challenges::Challenge;
pub static challengeBody: &'static str = r#"
"#;

pub static challengeTitle: &'static str = "hack_the_gibson";

pub static challengeAnswer: &'static str = "zlZ6QPh97sg16ds856RK0DIK1waNJwy";

pub fn generate<'a>(has_answered: &'a bool) -> Challenge {
    let answer = if *has_answered {
        Some(challengeAnswer)
    } else {
        None
    };

    Challenge {
        id: 6,
        challengeTitle: challengeBody,
        challengeBody,
        challengeAnswer: answer,
        score: 20,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer == challengeAnswer {
        true
    } else {
        false
    }
}
