use time;

mod facebook;
mod twitter;
mod google;

use diesel::prelude::*;
use models::users::User;
use schema::users::dsl::*;

use config::Config;

use rocket::{State, Response};
use rocket_contrib::{JSON, Value};

use endpoints::helpers::*;
use endpoint_error::EndpointResult;
use endpoints::pagination::Pagination;

use jwt::errors::{self, ErrorKind};
use jwt::{ encode, decode, Header, Algorithm, Validation };

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    pub name: String,
    // Username is the only key, drop hashids maybe ?
    pub username: String,
    pub avatar: String,
    //verified: bool,
    /// Prevent faked accounts.
    //swag_verified: Option<bool>,
    /// Issued at
    pub iat: i64,
    /// Expiry datetime
    pub exp: i64,
}

// only has_role() is used in this demo
impl UserToken {
    fn is_expired(&self) -> bool {
        let now = time::get_time().sec;
        now >= self.exp
    }

    fn is_claimed_user(&self, claimed_user: String) -> bool {
        self.username == claimed_user
    }
}

// Generate_token, creates the jwt key
pub fn generate_token(config: Config, user: User) -> Result<String, errors::Error> {
    let jwt_secret = config.Jwt.secret;
    let now = time::get_time().sec;
    let payload = UserToken {
        name: user.name,
        username: user.username,
        avatar: user.avatar,
        iat: now,
        exp: now + (60 * 60 * 24 * 7) // 1 week
    };
    encode(&Header::default(), &payload, jwt_secret.as_bytes())
}

/// This is what the refresh token received.
#[derive(Deserialize, Debug)]
pub struct Credential {
    pub accessToken: String
}

/// This is used to generate the JWT token, sign in mode
#[post("/refresh_token", format="application/json", data="<access_token>")]
pub fn refresh_token(config: State<Config>, access_token: JSON<Credential>)
    -> EndpointResult<JSON<Value>>
{
    let jwt_secret = &config.Jwt.secret;
    let decoded_data = decode::<UserToken>(&access_token.0.accessToken, jwt_secret.as_bytes(), &Validation::default()).unwrap();

    let now = time::get_time().sec;
    let payload = UserToken {
        name: decoded_data.claims.name,
        username: decoded_data.claims.username,
        avatar: decoded_data.claims.avatar,
        iat: now,
        exp: now + (60 * 60 * 24 * 7) // 1 week
    };

    let token = encode(&Header::default(), &payload, jwt_secret.as_bytes()).unwrap();

    Ok(JSON(json!({
        "success": true,
        "access_token": token,
    })))
}

#[derive(Deserialize)]
pub struct OauthCode {
    pub code: String,
}

#[post("/auth/facebook", format="application/json", data="<oauth_code>")]
pub fn facebook_oauth(config: State<Config>, oauth_code: JSON<OauthCode>)
    -> EndpointResult<JSON<Value>>
{
    let result = match facebook::auth(&*config, oauth_code.0.code.to_owned()) {
        Ok(token) => json!({
            "success": true,
            "accessToken": token,
        }),
        Err(reason) => json!({
            "success": false,
            "message": reason
        })
    };
    Ok(JSON(result))
}

#[post("/auth/twitter", format="application/json", data="<oauth_code>")]
pub fn twitter_oauth(config: State<Config>, oauth_code: JSON<OauthCode>)
    -> EndpointResult<JSON<Value>>
{
    let result = match twitter::auth(&*config, oauth_code.0.code.to_owned()) {
        Ok(token) => json!({
            "success": true,
            "accessToken": token
        }),
        Err(reason) => json!({
            "success": false,
            "message": reason
        })
    };
    Ok(JSON(result))
}

#[post("/auth/google", format="application/json", data="<oauth_code>")]
pub fn google_oauth(config: State<Config>, oauth_code: JSON<OauthCode>)
    -> EndpointResult<JSON<Value>>
{
    let result = match google::auth(&*config, oauth_code.0.code.to_owned()) {
        Ok(token) => json!({
            "success": true,
            "accessToken": token
        }),
        Err(reason) => json!({
            "success": false,
            "message": reason
        })
    };
    Ok(JSON(result))
}
