use crate::common::Result;
use crate::entity::user::{NewUserTab, UserTab};
use crate::schema::user_tab::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{RunQueryDsl, SqliteConnection};

pub fn create_user(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    tab: &NewUserTab,
) -> Result<i64> {
    let result = diesel::insert_into(user_tab).values(tab).execute(conn)?;
    Ok(result as i64)
}

pub fn get_user_by_userid(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    userid: i64,
) -> Result<UserTab> {
    let result = user_tab.filter(id.eq(userid)).first::<UserTab>(conn)?;
    Ok(result)
}

pub fn get_user_by_username(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    _username: &str,
) -> Result<UserTab> {
    let result = user_tab
        .filter(username.eq(_username))
        .first::<UserTab>(conn)?;
    Ok(result)
}
