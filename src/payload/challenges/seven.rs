use super::challenges::Challenge;
pub static challengeBody: &'static str = r#"
<p>
  You are are R2-D2, a cute robot from the Star Wars franchise are R2-D2. You
  are tasked with hacking into the most secure valut of the Death Star to
  retrive it's blueprints. The vault is controlled by a computer and requires a
  password to open. The source code for the vault was obtained by one of our
  comrades.
  <br />
  Please find the password(the flag) hidden in the source code and help the
  rebels live to fight another day.
  <br />
  <br />
  <br />
  Vault:
  <a href="https://deathstarvault.herokuapp.com/" target="_blank"
    >https://deathstarvault.herokuapp.com/</a
  >
  <br />
  Source code:
  <a href="https://github.com/realaravinth/death-star" target="_blank">https://github.com/realaravinth/death-star</a>
</p>
"#;

pub static challengeTitle: &'static str = "R2^_^3_&s%";

pub static challengeAnswer: &'static str = "nCTF{91QGh7kJxl0bBYt3tu96GnfqN8i9oSI}";

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
        id: 7,
        score: 70,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer == challengeAnswer {
        true
    } else {
        false
    }
}
