use time;
use rocket_contrib::{JSON, Value};
use jwt::errors::{ErrorKind};
use jwt::{encode, decode, Header, Algorithm, Validation};

mod hashids;
mod facebook;
mod twitter;
mod google;

static KEY: &'static str = "secret";

#[derive(Serialize, Deserialize, Debug)]
struct UserToken {
    // Username is the only key, drop hashids maybe ?
    username: String,
    avatar: String,
    verified: bool,
    /// Prevent faked accounts.
    swag_verified: Option<bool>,
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

/////////////////////////////////////////////////

fn jwt_generate(payload: UserToken) -> Result<String, String> {
    let now = time::get_time().sec;
    
    encode(&Header::default(), &payload, KEY.as_ref())?
}

///////////////////////////////////////////////////

/// This is what the refresh token received.
#[derive(Serialize, Deserialize, Debug)]
struct Credential {
    accessToken: String
}

/// This is used to generate the JWT token, sign in mode
#[post("/refresh_token", format="application/json", data="<access_token>")]
fn refresh_token(access_token: JSON<Credential>) -> JSON<Value> {

    let decoded_data = match decode::<UserToken>(&access_token,
                                                 key.as_ref(),
                                                 Algorithm::HS256,

    JSON(json!({
        "success": true,
        "access_token": "12345678"
    }))
    //let token_data = decode::<UserToken>(&access_token, KEY, Algorithm::HS256).unwrap();
}

/// This is what the oauth function received.
#[derive(Serialize, Deserialize, Debug)]
struct OauthCode {
    /// The authorized client code sent from client-side.
    code: String,
}

#[post("/auth/facebook", format="application/json", data="<oauth_code>")]
fn facebook_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    let result = match facebook::auth(oauth_code.code.to_owned()) {
        Ok(token) => json!({
            "success": true,
            "accessToken": token,
            "profile": "unimplemented"
        }),
        Err(reason) => json!({
            "success": false,
            "message": reason
        })
    };
    JSON(result)
}

#[post("/auth/twitter", format="application/json", data="<oauth_code>")]
fn twitter_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    let result = match twitter::auth(oauth_code.code.to_owned()) {
        Ok(token) => json!({
            "success": true,
            "accessToken": token
        }),
        Err(reason) => json!({
            "success": false,
            "message": reason
        })
    };
    JSON(result)
}

#[post("/auth/google", format="application/json", data="<oauth_code>")]
fn google_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    let result = match google::auth(oauth_code.code.to_owned()) {
        Ok(token) => json!({
            "success": true,
            "accessToken": token
        }),
        Err(reason) => json!({
            "success": false,
            "message": reason
        })
    };
    JSON(result)
}

