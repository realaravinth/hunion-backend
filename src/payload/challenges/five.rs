use super::challenges::Challenge;
pub static challengeAnswer: &'static str = "SC2INcxcddmV2";
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
    let challengeBody = r#" <p> The flag is hidden in this <a href='./voodoo.txt' target='_blank'>file</a>. It would be very tedious if you look for it manually, something tells me there's a better way. </p> <br /> <br /> File: <a href='./voodoo.txt' target='_blank'>voodoo.txt</a>. "#;
    let challengeTitle = "hidd3n_in_p1ain_sight";
    let ans = "SC2INcxcddmV2";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        challengeTitle,
        challengeBody,
        challengeAnswer: answer,
        score: 10,
        id: 5,

        hasAnswered: *has_answered,
    }
}
use super::challenges::CheckResponseResponseBuilder;
pub fn check(user_answer: &str) -> CheckResponseResponseBuilder {
    if user_answer.trim() == challengeAnswer {
        CheckResponseResponseBuilder {
            isCorrect: true,
            score: UNANSWERED.score as i32,
        }
    } else {
        CheckResponseResponseBuilder {
            isCorrect: false,
            score: UNANSWERED.score as i32,
        }
    }
}
