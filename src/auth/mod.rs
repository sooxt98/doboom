use time;
use rocket_contrib::{JSON, Value};
use jwt::{ encode, decode, Header, Algorithm };

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
        Ok(T) => T,
        Err(reason) => panic!(reason)
    };

    token
}

#[derive(Serialize, Deserialize, Debug)]
struct OauthCode {
    code: String
}

#[post("/auth/facebook", format="application/json", data="<oauth_code>")]
fn facebook_oauth(oauth_code: JSON<OauthCode>) -> JSON<Value> {
    println!("See what you got: {:?}", oauth_code.code);
    JSON(json!({ "status": true, "message": "just test" }))
}