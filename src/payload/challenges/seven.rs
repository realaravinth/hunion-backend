use super::challenges::Challenge;
pub static challengeAnswer: &'static str = "91QGh7kJxl0bBYt3tu96GnfqN8i9oSI";
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
    let challengeBody = r" <p> You are R2-D2, a cute robot from the Star Wars franchise. You are tasked with hacking into the most secure vault of the Death Star to retrieve it's blueprints. The vault is controlled by a computer and requires a password to open. The source code for the vault was obtained by one of our comrades. <br /> Please find the password(the flag) hidden in the source code and help the rebels live to fight another day. <br /> <br /> <br /> Vault: <a href='https://deathstarvault.herokuapp.com/' target='_blank' >https://deathstarvault.herokuapp.com/</a > <br /> Source code: <a href='https://github.com/realaravinth/death-star' target='_blank'>https://github.com/realaravinth/death-star</a> </p> ";
    let challengeTitle = "R2^_^3_&s%";
    let ans = "nCTF{91QGh7kJxl0bBYt3tu96GnfqN8i9oSI}";

    let answer = if *has_answered { Some(ans) } else { None };

    Challenge {
        challengeTitle,
        challengeBody,
        challengeAnswer: answer,
        id: 7,
        score: 70,

        hasAnswered: *has_answered,
    }
}
use super::challenges::CheckResponseResponseBuilder;
pub fn check(user_answer: &str) -> CheckResponseResponseBuilder {
    if user_answer.trim() == challengeAnswer {
        CheckResponseResponseBuilder {
            isCorrect: true,
            score: UNANSWERED.score as i32,
        }
    } else {
        CheckResponseResponseBuilder {
            isCorrect: false,
            score: UNANSWERED.score as i32,
        }
    }
}
