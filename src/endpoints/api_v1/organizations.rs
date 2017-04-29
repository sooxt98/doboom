use db::{DB, DbError};
use diesel::prelude::*;

use database::models::Organization;
use database::models::UpdatedOrganization;
use database::schema::organization::dsl::*;

use rocket::{State, Response};
use rocket_contrib::{JSON, Value};

use auth::decode_token;
use endpoints::helpers::*;
use endpoint_error::EndpointResult;
use endpoints::pagination::Pagination;

use endpoints::api_v1::users::UserProfile;
use endpoints::api_v1::products::ProductProfile;

#[derive(Deserialize, Serialize)]
pub struct OrganizationProfile {
    pub name: String,
    pub avatar: Option<String>,
    pub description: String,
    pub published: bool,
    pub member: Vec<OrganizationProfile>, // username
    pub products: Vec<ProductProfile>,
}

/// Return the organization profile
#[get("/organization/profile/<name>", format = "application/json")]
fn get_organization(db: State<DB>, name: String) -> Endpoint<JSON<OrganizationProfile>> {
    let conn = &*db.pool().get()?;

    let org = organizations.find(name).first::<Organization>(conn)?;

    let profile = OrganizationProfile {
        name: org.name,
        avatar: org.avatar,
        description: org.description,
        published: org.published,
        member: vec![],
        products: vec![],
    };

    Ok(JSON(profile))
}

/// Edit current user's profile
#[put("/organization/profile/<name>/edit", data = "<updated_profile>", format = "application/json")]
fn edit_profile(db: State<DB>, req: &Request, name: String) -> EndpointResult<JSON<OrganizationProfile>> {
    match decode_token(req.headers.get_one("Authorization")) {
        Some (token) => {
            let conn = &*db.pool().get()?;

            let user = diesel::update(users.find(token.username)).set(&updated_profile.0)
                .get_result::<Organization>(conn)?;

            let profile = OrganizationProfile {
                name: org.name,
                avatar: org.avatar,
                description: org.description,
                published: org.published,
                member: vec![],
                products: vec![],
            };

            Ok(JSON(profile))
        }
        None => Response::build().status(Status::Unauthorized).ok()
    }
}

/// Query all the organization from a username given
pub fn list_organizations(db: &Db, username: String, pagination: Option<Pagination>)
    -> Vec<OrganizationProfile>
{
    let conn = &*db.pool().get()?;
    // TODO
    None
}
