use time;
use rocket_contrib::{JSON, Value};
use jwt::{ encode, decode, Header, Algorithm };

mod facebook;
// TODO: mod twitter;
// TODO: mod google;

static KEY: &'static str = "secret";

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct UserToken {
    // issued at
    iat: i64,
    // expiration
    exp: i64,
    user: String,
    userid: String,
}

// only has_role() is used in this demo
impl UserToken {
    fn is_expired(&self) -> bool {
        let now = time::get_time().sec;
        now >= self.exp
    }

    fn is_claimed_user(&self, claimed_user: String) -> bool {
        self.userid == claimed_user
    }
}

fn jwt_generate(user: String, userid: String) -> String {
    let now = time::get_time().sec;
    let payload = UserToken {
        iat: now,
        exp: now + (60 * 60 * 24 * 7), // One week time range
        user: user,
        userid: userid
    };

    let token = match encode(Header::default(), &payload, KEY.as_ref()) {
        Ok(t) => t,
        Err(reason) => panic!(reason)
    };

    token
}

/// This is what the oauth function received.
#[derive(Serialize, Deserialize, Debug)]
struct OauthCode {
    /// The authorized client code sent from client-side.
    code: String
}

#[post("/auth/facebook", format="application/json", data="<oauth_code>")]
fn facebook_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    let result = match facebook::auth() {
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

/* TODO
#[post("/auth/twitter", format="application/json", data="<oauth_code")]
fn twitter_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    let result = match twiter::auth(oauth_code.code) {
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

#[post("/auth/google", format="application/json", data="<oauth_code")]
fn google_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    let result = match google::auth(oauth_code.code) {
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
**/