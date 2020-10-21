use super::challenges::Challenge;

pub static challengeBody: &'static str = r#"
  <p id="Obi-Wan-Kenobi">
  To a computer, everything is either a 1 or a 0. Even if you feed it an image
  of pizzas, it'll still be a bunch of 0s and 1s. This way of representing
  things is called base-2 numeral system or binary numeral system. The flag for
  this challenge is a 10-digit number in the decimal number system(ranges from 0
  to 9). <br /> <br />We have given your computer special instructions to hide it away from
  you. But you know how computers are,they are high maintenance. So maybe if you
  ask it nicely, it might tell you what the flag is!
</p>
<br />
<style>
  .hidden {
    visibility: hidden;
  }
  .show {
    visibility: visible;
  }
</style>
"#;

pub static challengeTitle: &'static str = "be3p_b00p_be3p_b00p";

pub static challengeAnswer: &'static str = "7641648545";

pub fn generate<'a>(has_answered: &'a bool) -> Challenge {
    let answer = if *has_answered {
        Some(challengeAnswer)
    } else {
        None
    };

    Challenge {
        id: 1,
        challengeTitle: challengeBody,
        challengeBody,
        challengeAnswer: answer,
        score: 10,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer == challengeAnswer {
        true
    } else {
        false
    }
}
