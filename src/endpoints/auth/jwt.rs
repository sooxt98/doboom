use time;
use std::env;
use jwt::errors;
use jwt::{encode, decode, Header, Algorithm, Validation};

#[derive(Serialize, Deserialize, Debug)]
struct UserToken {
    name: String,
    /// Username is the only key, drop hashids.
    username: String,
    avatar: Option<String>,
    /// Issued at
    iat: i64,
    /// Exipirt datetime
    exp: i64
}

impl UserToken {
    fn is_expired(&self) -> bool {
        let now = time::get_time().sec;
        now >= self.exp
    }

    fn is_claimed_user(&self, claimed_user: String) -> bool {
        self.username == claimed_user
    }
}

/// Generate token, creates the jwt key
pub fn generate_token(
    name: String,
    username: String,
    avatar: Option<String>,
    exp: i64
) -> Result<String, errors::Error>
{
    let now = time::get_time().sec;
    let key = "secret";
    let jwt = UserToken {
        name: name,
        username: username,
        avatar: avatar,
        iat: now,
        exp: now + TWO_HOURS,
    };

    encode(&Header::default(), &jwt, key.as_ref())
}

/// Decode token, return the payload
pub fn decode_token(token: String) -> Option<UserToken> {
    let key = env::var("DOBOOM_SECRET_KEY");
    match decode::<UserToken>(&token, key.unwrap().as_ref(), &Validation::default()) {
        Ok(c) => Some(c.claims),
        Err(_) => None
    }
}
