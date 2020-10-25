use super::challenges::Challenge;

pub static CHALLENGE_ANSWER: &'static str = "adfadsfalkj23423";
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
    let challenge_body: &'static str = r#" <p> Did you know that files have fingerprints just like us? They <em>are</em> just 0s and 1s but they too have fingerprints. The entropy(randomness) lies is how their contents(0s and 1s) are arranged. This fingerprint is often calculated using hash functions, such as SHA256. </p> <p> The flag for this challenge is the SHA256 checksum(fingerprint) of this <a href='/file.txt' target='_blank'>file</a> calculated using SHA256 hash function. <br /> <br /> File: <a href='/file.txt' target='_blank'>file.txt</a> </p> "#;
    let challenge_title: &'static str = "iam_uniqu3";
    let ans = "db108f489f3b14e228b3b35f365b3b6d4f64a6f653287347ad3bde203c70cae7";
    let answer = if *has_answered { Some(ans) } else { None };
    Challenge {
        id: 2,
        challenge_body,
        challenge_title,
        challenge_answer: answer,
        score: 10,

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
