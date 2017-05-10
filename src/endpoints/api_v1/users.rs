use database::{Db, DbError};
use config::Config;

use diesel;
use diesel::prelude::*;

use models::users::*;
use schema::users::dsl::*;

use rocket::{State, Response};
use rocket_contrib::{JSON, Value};

// use auth::decode_token;
use endpoints::helpers::*;
use endpoint_error::EndpointResult;
use endpoints::pagination::Pagination;

//use endpoints::api_v1::organizations::{list_organizations, OrganizationProfile};
//use endpoints::api_v1::products::{list_product, ProductProfile};

#[derive(Deserialize, Serialize)]
pub struct UserProfile {
    pub user: String,
    pub username: String,

    pub avatar: String,

    //pub organizations: Vec<OrganizationProfile>,
    //pub products: Vec<ProductProfile>,
}

/// Return the user profile for the user profile page.
#[get("/user/profile/<user_name>", format = "application/json")]
pub fn get_profile(db: State<Db>, user_name: String) -> EndpointResult<JSON<UserProfile>> {
    let conn = &*db.pool().get()?;

    let user = users.filter(username.eq(user_name)).first::<User>(conn)?;

    let profile = UserProfile {
        user: user.name,
        username: user.username,
        avatar: user.avatar,
        //organization: vec![], //list_organizations(db, user.username),
        //products: vec![], //list_product(db, user.username),
    };

    Ok(JSON(profile))
}

/*
/// Edit current user's profile
#[put("/user/profile/edit", format = "application/json", data = "<updated_user>")]
fn edit_profile(db: State<DB>, req: &Request, updated_user: ) -> EndpointResult<JSON<UserProfile>> {
    match decode_token(req.headers.get_one("Authorization")) {
        Some (token) => {
            let conn = &*db.pool().get()?;

            let user = diesel::update(users.eq(token.username)).set(&updated_user.0)
                .get_result::<User>(conn)?;

            let profile = UserProfile {
                user: user.name,
                username: user.username,
                avatar: user.avatar,
                organizations: vec![], //list_organizations(db, user.username),
                products: vec![], //list_product(db, user.username),
            };

            Ok(JSON(profile))
        }
        None => Response::build().status(Status::Unauthorized).ok()
    }
}
*/
