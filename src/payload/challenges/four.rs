use super::challenges::Challenge;
pub static challengeBody: &'static str = r#"
<p>
  We wanted this challenge to be very easy. In fact, we wanted to just give the
  flag away without any ceremony. Because of how simple this challenge is, we
  figured we'll put our laziest server up to this task. We asked it to just give
  it away when you ask for it. But it looks like it is feeling particularly
  energetic tonight and is pulling tricks on us. We are sorry but you are going
  to have to retrieve this flag to clear this challenge.
  <br />
  <br />
  Misbehaving server:
  <a href='${HEARTBEAT_LEADER}'>${HEARTBEAT_LEADER}</a>
  <br />
  <br />
  P.S: Apparently, Firefox is fast enough to talk to the misbehaving server!
</p>
"#;

pub static challengeTitle: &'static str = "catch_m3_if_u_cancatch_m3_if_u_can";

pub static challengeAnswer: &'static str = "qEizblnviY2fBtApKgQjf08Wdr9S";

pub fn generate<'a>(has_answered: &'a bool) -> Challenge {
    let answer = if *has_answered {
        Some(challengeAnswer)
    } else {
        None
    };

    Challenge {
        challengeTitle: challengeBody,
        challengeBody,
        challengeAnswer: answer,
        score: 20,
        id: 4,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer == challengeAnswer {
        true
    } else {
        false
    }
}
