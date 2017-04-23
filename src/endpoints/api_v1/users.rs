use db::{DB, DbError};
use diesel::prelude::*;
use database::models::clicks::Count;

use database::models::User;
use database::models::UpdatedUser;
use database::schema::users::dsl::*;

use rocket::{State, Response};
use rocket_contrib::{JSON, Value};

use auth::decode_token;
use endpoints::helpers::*;
use endpoint_error::EndpointResult;
use endpoints::pagination::Pagination;

#[derive(Deserialize, Serialize)]
pub struct UserProfile {
    pub user: String,
    pub username: String,

    pub avatar: Option<String>,

    pub organizations: Option<Vec<String>>,
    pub products: Option<Vec<String>>,
}

/// Return the user profile for the user profile page.
#[get("/user/profile/<username>", format = "application/json")]
fn get_profile(db: State<DB>, username: String) -> EndpointResult<JSON<UserProfile>> {
    let conn = &*db.pool().get()?;

    let user = users.find(username).first::<User>(conn)?;

    let profile = UserProfile {
        user: user.name,
        username: user.username,
        avatar: user.avatar,
        organization: None, // unimplemnted
        products: None,
    };

    Ok(JSON(profile))
}

/// Edit current user's profile
#[put("/user/profile/edit", data = "<updated_user>", format = "application/json")]
fn edit_profile(db: State<DB>, req: &Request) -> EndpointResult<JSON<UserProfile>> {
    match decode_token(req.headers.get_one("Authorization")) {
        Some (token) => {
            let conn = &*db.pool().get()?;

            let user = diesel::update(users.find(token.username)).set(&updated_user.0)
                .get_result::<User>(conn)?;

            let profile = UserProfile {
                user: user.name,
                username: user.username,
                avatar: user.avatar,
                organizations: organization_query(db, user.username),
                products: product_query(db, user.username),
            };

            Ok(JSON(profile))
        }
        None => Response::build().status(Status::Unauthorized).ok()
    }
}

/// Query all the organization from a username given
fn organization_query(db: &Db, username: String, pagination: Option<Pagination>)
    -> Option<Vec<Organization>>
{
    let conn = &*db.pool().get()?;
    // TODO
    None
}

/// Query all the product from a username given
fn product_query(db: &Db, username: String, pagination: Option<Pagination>)
    -> Option<Vec<Product>>
{
    let conn = &*db.pool().get()?;
    // TODO
    None
}