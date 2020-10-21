use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct User {
    pub userID: String,
    pub name: String,
    pub score: usize,
    pub progress: Progress,
}

pub type Progress = [bool; 7];
//    c1: bool,
//    c2: bool,
//    c3: bool,
//    c4: bool,
//    c5: bool,
//    c6: bool,
//    c7: bool,
//}

//impl Progress {
//    pub fn new() -> Self {
//        Progress [
//             false,
//             false,
//             false,
//             false,
//             false,
//             false,
//             false,
//        ]
//    }
//}

impl User {
    pub fn new(userID: String, name: String) -> Self {
        User {
            userID,
            name,
            score: 0,
            progress: [false; 7], //Progress::new(),
        }
    }
}
