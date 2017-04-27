#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[has_many(products)]
#[has_many(organizations)]
#[has_many(posts)]
#[has_many(comments)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub username: String,

    /// For 3-rd parties login only, so no password stored.
    // pub password: String,

    pub avatar: Option<String>,
    pub avatar_const: bool
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub username: String,
    pub email: String,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UpdatedUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[has_many(comments)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub published: bool,
    pub user_id: Option<i64>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub body: String,
    pub user_id: Option<i64>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct UpdatedPost {
    pub body: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub body: String,
    pub user_id: i64,
    pub post_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "comments"]
pub struct NewPostComment {
    pub body: String,
    pub user_id: i64,
    pub post_id: i32,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "comments"]
pub struct UpdatedPostComment {
    pub body: Option<String>,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[has_many(comments)]
pub struct Organization {
    pub id: i32,
    pub avatar: String,
    pub name: String,
    pub description: String,
    pub published: bool,
    pub user_id: Vec<i64>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "organizations"]
pub struct NewOrganization {
    pub name: String,
    pub description: String,
    pub user_id: Vec<i64>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "organizations"]
pub struct UpdatedOrganization {
    pub name: Option<String>,
    pub description: Option<String>,
}

/////////////////////////////////////////////////

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[has_many(comments)]
pub struct Product {
    pub id: i32,
    pub description: String,
    pub published: bool,
    pub user_id: Option<i64>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "products"]
pub struct NewProduct {
    pub description: String,
    pub user_id: Option<i64>,
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "products"]
pub struct UpdatedProduct {
    pub description: Option<String>,
}

/////////////////////////////////////////////////

use super::schema::posts;
use super::schema::users;
use super::schema::comments;
use super::schema::products;
use super::schema::organizations;
