use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::models::schema::users)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}
