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

use endpoints::api_v1::organizations::{list_organizations, OrganizationProfile};
use endpoints::api_v1::products::{list_product, ProductProfile};

#[derive(Deserialize, Serialize)]
pub struct UserProfile {
    pub user: String,
    pub username: String,

    pub avatar: Option<String>,

    pub organizations: Vec<OrganizationProfile>,
    pub products: Vec<ProductProfile>,
}

/// Return the user profile for the user profile page.
#[get("/user/profile/<username>", data="name"format = "application/json")]
fn get_profile(db: State<DB>, username: String) -> EndpointResult<JSON<UserProfile>> {
    let conn = &*db.pool().get()?;

    let user = users.find(username).first::<User>(conn)?;

    let profile = UserProfile {
        user: user.name,
        username: user.username,
        avatar: user.avatar,
        organization: list_organizations(db, user.username),
        products: list_product(db, user.username),
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
                organizations: list_organizations(db, user.username),
                products: list_product(db, user.username),
            };

            Ok(JSON(profile))
        }
        None => Response::build().status(Status::Unauthorized).ok()
    }
}