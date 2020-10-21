use super::challenges::Challenge;
pub static challengeBody: &'static str = r#"
<p>
  The flag is hidden in this <a href="./voodo.txt" target="_blank">file</a>. It
  would be very tedious if you look for it manually, something tells me there's
  a better way.
</p>
  <br />
  <br />
  File:
<a href="./voodo.txt" target="_blank">voodo.txt</a>.
"#;

pub static challengeTitle: &'static str = "hidd3n_in_p1ain_sight";

pub static challengeAnswer: &'static str = "SC2INcxcddmV2";

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
        score: 10,
        id: 5,
    }
}
