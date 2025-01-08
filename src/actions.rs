// actions.rs
use diesel::prelude::*;
use uuid::Uuid;
use crate::models::User;

type DbError = Box<dyn std::error::Error + Send + Sync>;
pub fn find_user_by_uid(conn: &mut PgConnection, uid: Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users.filter(id.eq(uid)).first::<User>(conn).optional()?;
    Ok(user)
}

pub fn insert_new_user(conn: &mut PgConnection, nm: &str, em: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = User {
        id: Uuid::new_v4(),
        name: nm.to_owned(),
        email: em.to_owned(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}


