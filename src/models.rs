use super::schema::users;
// use diesel::insertable::Insertable;
// use diesel::query_builder::query_id::QueryId;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub name: &'a str,
}
