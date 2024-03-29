use std::env;
use std::str::FromStr;

use futures::future;
use futures::{Future, Stream};

use ring::{digest, hmac};

use tokio_core::reactor::Core;

use serde_json::from_str;
use serde_json::Value as JsonValue;

use hyper_tls::HttpsConnector;
use hyper::{Uri, Method, Error};
use hyper::client::{Client, Request};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::{Authorization, Accept, qitem};

use config::Config;

#[derive(Serialize, Deserialize)]
struct CodeResp {
    access_token: String,
}

/// Communicate with the facebook
pub fn auth(config: &Config, code: String) -> Result<String, String> {
    let client_id = &config.FacebookOauth.app_id;
    let client_secret = &config.FacebookOauth.app_secret;
    let redirect_uri = &config.FacebookOauth.redirect_uri;

    let fields = vec!["email", "id", "last_name", "picture.type(large)"];
    let accessTokenUrl = "https://graph.facebook.com/oauth/access_token";
    let graphApiUrl = format!("https://graph.facebook.com/v2.5/me?fields={}", fields.join(","));

    let accessTokenUri = Uri::from_str(&format!("{}?code={}&client_id={}&client_secret={}&redirect_uri={}",
                                                accessTokenUrl
                                                , code
                                                , client_id
                                                , client_secret
                                                , redirect_uri)
                                       ).expect("Invalid query parameter");

    /// This is to ask for the accessToken from Facebook
    let mut code_req = Request::new(Method::Get, accessTokenUri);
    code_req.headers_mut().set(Accept(vec![qitem(
                Mime(TopLevel::Application,
                     SubLevel::Json,
                     vec![(Attr::Charset, Value::Utf8)]))
    ]));

    let mut evloop = Core::new().unwrap();
    let handle = evloop.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle))
        .build(&handle);

    let worker = client.request(code_req).and_then(|code_result| {
        println!("Response: {}", code_result.status());
        code_result.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, Error>(v)
        }).and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            let code_result_json: CodeResp = from_str(&s).unwrap();

            let hmac = fb_digest(code_result_json.access_token.as_str(), client_secret.as_str());
            let graphApiUri = Uri::from_str(&format!("{}?access_token={}&appsecret_proof={}",
                                                    graphApiUrl
                                                    , code_result_json.access_token
                                                    , hmac)
                                            ).expect("Invalid result");

            let mut graph_req = Request::new(Method::Get, graphApiUri);

            graph_req.headers_mut().set(Accept(vec![qitem(
                        Mime(TopLevel::Application,
                             SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)]))
            ]));

            client.request(graph_req).and_then(|graph_result| {
                println!("Response: {}", graph_result.status());
                graph_result.body().fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                }).and_then(|chunks| {
                    let s = String::from_utf8(chunks).unwrap();
                    let graph_result_json: JsonValue = from_str(&s).unwrap();
                    future::ok::<_, Error>(graph_result_json)
                })
            })
        })
    });

    let user_profile = evloop.run(worker).unwrap();

    println!("user_profile returned from facebook: {:?}", user_profile);
    Ok(String::from("ABC"))
}

// use to hash the secrect using the given access_token
fn fb_digest(data: &str, appsecret: &str) -> String {
    let signed_key = hmac::SigningKey::new(&digest::SHA256, appsecret.as_bytes());
    let signature = hmac::sign(&signed_key, data.as_bytes());

    String::from_utf8(signature.as_ref().to_owned()).unwrap()
}
