use super::challenges::Challenge;

pub static challengeAnswer: &'static str =
    "db108f489f3b14e228b3b35f365b3b6d4f64a6f653287347ad3bde203c70cae7";
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
    let challengeBody: &'static str = r#" <p> Did you know that files have fingerprints just like us? They <em>are</em> just 0s and 1s but they too have fingerprints. The entropy(randomness) lies is how their contents(0s and 1s) are arranged. This fingerprint is often calculated using hash functions, such as SHA256. </p> <p> The flag for this challenge is the SHA256 checksum(fingerprint) of this <a href='/file.txt' target='_blank'>file</a> calculated using SHA256 hash function. <br /> <br /> File: <a href='/file.txt' target='_blank'>file.txt</a> </p> "#;
    let challengeTitle: &'static str = "iam_uniqu3";
    let ans = "db108f489f3b14e228b3b35f365b3b6d4f64a6f653287347ad3bde203c70cae7";
    let Answer = if *has_answered { Some(ans) } else { None };
    Challenge {
        id: 2,
        challengeBody,
        challengeTitle,
        challengeAnswer: Answer,
        score: 10,

        hasAnswered: *has_answered,
    }
}

pub fn check(user_answer: &str) -> bool {
    if user_answer.trim() == challengeAnswer {
        true
    } else {
        false
    }
}
