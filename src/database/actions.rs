use crate::errors::*;
use crate::models::InsertableUser;
use crate::payload::register::RegisterRequestPayload;
use diesel::prelude::*;

pub fn find_user_by_userid(
    user_id: &str,
    conn: &PgConnection,
) -> ServiceResult<Option<InsertableUser>> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(userid.eq(user_id))
        .first::<InsertableUser>(conn)
        .optional()?;
    Ok(user)
}

pub fn insert_new_user(payload: RegisterRequestPayload, conn: &PgConnection) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    let insertable: InsertableUser = payload.into();
    diesel::insert_into(users)
        .values(insertable)
        .execute(conn)?;

    Ok(())
}

pub fn update_score(add_score: i32, challenge: u32, conn: &PgConnection) -> ServiceResult<()> {
    match challenge {
        1 => Ok(set_one(conn, add_score)?),
        2 => Ok(set_two(conn, add_score)?),
        3 => Ok(set_three(conn, add_score)?),
        4 => Ok(set_four(conn, add_score)?),
        5 => Ok(set_five(conn, add_score)?),
        6 => Ok(set_six(conn, add_score)?),
        7 => Ok(set_seven(conn, add_score)?),
        _ => Err(ServiceError::InternalServerError),
    }
}

pub fn get_top_twenty(conn: &PgConnection) -> ServiceResult<Vec<InsertableUser>> {
    use crate::schema::users::dsl::*;
    let results = users
        .filter(score.gt(0))
        .order(score.desc())
        .limit(20)
        .load::<InsertableUser>(conn)?;

    Ok(results)
}

fn set_one(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), one.eq(true)))
        .execute(conn)?;
    Ok(())
}

fn set_two(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), two.eq(true)))
        .execute(conn)?;
    Ok(())
}
fn set_three(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), three.eq(true)))
        .execute(conn)?;
    Ok(())
}
fn set_four(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), four.eq(true)))
        .execute(conn)?;
    Ok(())
}
fn set_five(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), five.eq(true)))
        .execute(conn)?;
    Ok(())
}
fn set_six(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), six.eq(true)))
        .execute(conn)?;
    Ok(())
}
fn set_seven(conn: &PgConnection, add_score: i32) -> ServiceResult<()> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .set((score.eq(score + add_score), seven.eq(true)))
        .execute(conn)?;
    Ok(())
}
