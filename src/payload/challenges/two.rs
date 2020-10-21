use super::challenges::Challenge;

pub static challengeBody: &'static str = r#"
<p>
Did you know that files have fingerprints just like us? They <em>are</em> just 0s and
  1s but they too have fingerprints. The entropy(randomness) lies is how their
  contents(0s and 1s) are arranged. This fingerprint is often calculated using
  hash functions, such as SHA256.
</p>
<p>
  The flag for this challenge is the SHA256 checksum(fingerprint) of this
  <a href="/file.txt" target="_blank">file</a> calculated using SHA256 hash function.
  <br />
  <br />
  File: 
  <a href="/file.txt" target="_blank">file.txt</a>
</p> "#;

pub static challengeTitle: &'static str = "iam_uniqu3";
pub static challengeAnswer: &'static str =
    "db108f489f3b14e228b3b35f365b3b6d4f64a6f653287347ad3bde203c70cae7";

pub fn generate<'a>(has_answered: &'a bool) -> Challenge {
    let Answer = if *has_answered {
        Some(challengeAnswer)
    } else {
        None
    };
    Challenge {
        id: 2,
        challengeBody,
        challengeTitle,
        challengeAnswer: Answer,
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
