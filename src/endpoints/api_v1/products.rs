use db::{DB, DbError};
use diesel::prelude::*;

use database::models::Product;
use database::models::UpdatedProduct;
use database::schema::product::dsl::*;

use rocket::{State, Response};
use rocket_contrib::{JSON, Value};

use auth::decode_token;
use endpoints::helpers::*;
use endpoint_error::EndpointResult;
use endpoints::pagination::Pagination;

use endpoints::api_v1::users::UserProfile;
use endpoints::api_v1::organizations::OrganizationProfile;

#[derive(Deserialize, Serialize)]
pub struct ProductProfile {
    pub id: String, // hashed ids
    pub name: String,
    pub description: String,
    pub avatar: String,
    pub images: Vec<String>,
    pub maker: Vec<UserProfile>,
    pub under_organizations: Vec<OrganizationProfile>,
}

/// Return the organization profile
#[get("/organization/profile/<name>", format = "application/json")]
fn get_product(db: State<DB>, name: String) -> Endpoint<JSON<ProductProfile>> {
    let conn = &*db.pool().get()?;

    let product = products.find(name).first::<Product>(conn)?;

    let maker = products.find()
    let profile = ProductProfile {
        id: format!("{}", product.id)
        name: product.name,
        description: product.description,
        avatar: product.avatar,
        images: product.maker,
        maker:
        under_organizations: vec![],
    }

    Ok(JSON(profile))
}

/// Edit current user's profile
#[put("/organization/profile/<name>/edit", data = "<updated_product>", format = "application/json")]
fn edit_profile(db: State<DB>, req: &Request, name: String) -> EndpointResult<JSON<OrganizationProfile>> {
    match decode_token(req.headers.get_one("Authorization")) {
        Some (token) => {
            let conn = &*db.pool().get()?;

            let user = diesel::update(users.find(updated_product.name)).set(&updated_product.0)
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

/// Query all the product from a username given
pub fn list_product(db: &Db, username: String, pagination: Option<Pagination>)
    -> Option<Vec<ProductProfile>>
{
    let conn = &*db.pool().get()?;
    // TODO
    None
}