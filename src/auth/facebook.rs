
pub fn auth() -> Result<String, String> {
    println!("Hello darkness my old friend");
    Ok(String::from("ABC"))
}

/*
use ring::{digest, hmac, rand};
use hyper::{Client};

pub fn auth(code: String) -> Result<String, String> {
    let fields = vec!["email", "id", "last_name", "picture.type(large)"];
    let accessTokenURL = "https://graph.facebook.com/oauth/access_token";

    let graphApiURL = format!("https://graph.facebook.com/v2.5/me?fields={}", fields.join(","));

    let c = Client::new();

    let respCode = c.get(format!("{}?code={}&client_id={}&redirect_uri={}", accessTokenURL, code, CLIENT_ID, REDIRECT_URI)).send()?;

    // The appsecret_proof needed by facebook
    let appsecret = [0u8; 32];
    let signing_key = hmac::SigningKey::new(&digest::SHA256, appsecret.as_ref());
    let hmac = hmac::sign(&signing_key, respCode.access_token);
    let hmac_digest = to_hex(hmac.as_ref());

    let graphCode = c.get(format!("{}?access_token={}&appsecret_proof={}", graphApiURL, respCode.access_token, hmac_digest)).send()?;

    println!("{:?}", graphCode);
    Ok(String::from("ABC"))
}

// use as_ref to get the value as a &[u8]
fn to_hex (arr: &[u8]) -> String {
    let strs = arr.into_iter();
    strs.iter().map(|x| format!("{:02X}", x)).collect();
    strs.connect(" ")
}
*/
