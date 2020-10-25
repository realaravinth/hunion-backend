use super::challenges::Challenge;
pub static CHALLENGE_ANSWER: &'static str = "qEizblnviY2fBtApKgQjf08Wdr9S";
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
    let challenge_body = r#" <p> We wanted this challenge to be very easy. In fact, we wanted to just give the flag away without any ceremony. Because of how simple this challenge is, we figured we'll put our laziest server up to this task. We asked it to just give it away when you ask for it. But it looks like it is feeling particularly energetic tonight and is pulling tricks on us. We are sorry but you are going to have to retrieve this flag to clear this challenge. <br /> <br /> Misbehaving server: <a href='http://batsense.net:8081'>batsense.net:8081</a> <br /> <br /> P.S: Apparently, Firefox is fast enough to talk to the misbehaving server! </p> "#;
    let challenge_title = "catch_m3_if_u_cancatch_m3_if_u_can";
    let ans = "qEizblnviY2fBtApKgQjf08Wdr9S";
    let answer = if *has_answered { Some(ans) } else { None };
    Challenge {
        challenge_title,
        challenge_body,
        challenge_answer: answer,
        score: 20,
        id: 4,

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
