use super::challenges::Challenge;
pub static CHALLENGE_ANSWER: &'static str = "SC2INcxcddmV2";
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
    let challenge_body = r#" <p> The flag is hidden in this <a href='./voodoo.txt' target='_blank'>file</a>. It would be very tedious if you look for it manually, something tells me there's a better way. </p> <br /> <br /> File: <a href='./voodoo.txt' target='_blank'>voodoo.txt</a>. "#;
    let challenge_title = "hidd3n_in_p1ain_sight";
    let ans = "SC2INcxcddmV2";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        challenge_title,
        challenge_body,
        challenge_answer: answer,
        score: 10,
        id: 5,

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
