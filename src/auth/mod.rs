extern crate time;
extern crate rustc_serialize;
extern crate jsonwebtoken as jwt;

use jwt::{ encode, decode, Header, Algorithm };

static KEY: &'static [u8; 16] = "1234567890123456";

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

pub fn jwt_generate(user: String, userid: String) -> String {
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
    }

    token
}
