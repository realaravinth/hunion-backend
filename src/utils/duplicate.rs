use crate::errors::*;
use crate::models::InsertableUser;
pub fn detect_duplicate(user: InsertableUser, challenge: u32) -> ServiceResult<bool> {
    let duplicate = match challenge {
        1 => user.one,
        2 => user.two,
        3 => user.three,
        4 => user.four,
        5 => user.five,
        6 => user.six,
        7 => user.seven,
        _ => return Err(ServiceError::InternalServerError),
    };
    if duplicate.is_none() {
        return Ok(true);
    } else {
        return Err(ServiceError::AlreadyAnswered);
    }
}
