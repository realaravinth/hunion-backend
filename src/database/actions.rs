use crate::errors::*;
use crate::models::InsertableUser;
use crate::payload::register::RegisterRequestPayload;
use diesel::prelude::*;

pub fn find_user_by_userid(
    userID: &str,
    conn: &PgConnection,
) -> ServiceResult<Option<InsertableUser>> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(userid.eq(userid))
        .first::<InsertableUser>(conn)
        .optional()?;
    Ok(user)
}

pub fn insert_new_user(
    // prevent collision with `name` column imported inside the function
    payload: RegisterRequestPayload,
    conn: &PgConnection,
) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    let insertable: InsertableUser = payload.into();
    diesel::insert_into(users)
        .values(insertable)
        .execute(conn)?;

    Ok(())
}
