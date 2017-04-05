#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,

    pub avatar: String,
    pub avatar_const: bool,

    pub under_organization: Vec<String>,
    pub product: Vec<String>,
}
