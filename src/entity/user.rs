use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = crate::schema::user_tab)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserTab {
    pub id: i64,
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = crate::schema::user_tab)]
pub struct NewUserTab {
    pub nickname: String,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub create_time: i64,
    pub update_time: i64,
}

#[derive(Insertable, AsChangeset, Debug, Clone)]
#[diesel(table_name = crate::schema::user_tab)]
pub struct UpdateUserTab {
    pub nickname: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub update_time: i64,
}
