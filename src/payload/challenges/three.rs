use super::challenges::Challenge;
pub static challengeBody: &'static str = r#"
<p>
  What does "RTV3Jld3djFqUlZSdUhUXnlmOCYl" mean? I think it has something to do
  with a binary-to-text encoding that's popular on the World Wide Web
</p>
"#;

pub static challengeTitle: &'static str = ".... --- .-- -.. -.--";

pub static challengeAnswer: &'static str = "E5w&Wwv1jRVRuHT^yf8&%";

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
        id: 3,
    }
}
