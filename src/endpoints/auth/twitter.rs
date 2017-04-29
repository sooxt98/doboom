use std::env;
use futures::future;
use std::str::FromStr;
use serde_json::from_str;
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::{Uri, Method, Error};
use serde_json::Value as JsonValue;

use base64::encode;

use hyper::client::{Client, Request};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::{Accept, Bearer, Basic, Headers, Authorization, ContentType, qitem};

#[derive(Serialize, Deserialize)]
struct CodeResp {
    access_token: String,
}

pub fn auth(code: String) -> Result<String, String> {
    
    let consumerKey = env::var("TWITTER_CONSUMERKEY").expect("TWITTER_CONSUMERKEY must be set");
    let consumerSecret = env::var("TWITTER_CONSUMERSECRET").expect("TWITTER_CONSUMERSECRET must be set");

    let accessTokenUrl = Uri::from_str("https://api.twitter.com/oauth2/token").unwrap();
    let accountApiUrl = Uri::from_str("https://api.twitter/1.1/account/verify_credentials.json?include_email=true").unwrap();

    let _credential = format!("{}:{}", consumerKey, consumerSecret);
    let credential = encode(_credential.as_bytes());

    let mut code_req = Request::new(Method::Post, accessTokenUrl);

    code_req.headers_mut().set(Accept(vec![qitem(
                Mime(TopLevel::Application,
                     SubLevel::Json,
                     vec![(Attr::Charset, Value::Utf8)]))
    ]));
    code_req.headers_mut().set(Authorization(Basic {
        username: credential.to_owned(),
        password: None
    }));

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

            let mut account_req = Request::new(Method::Post, accountApiUrl);

            account_req.headers_mut().set(Accept(vec![qitem(
                        Mime(TopLevel::Application,
                             SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)]))
            ]));

            account_req.headers_mut().set(Authorization(Bearer {
                token: code_result_json.access_token.to_owned()
            }));

            client.request(account_req).and_then(|account_result| {
                println!("Response: {}", account_result.status());
                account_result.body().fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                }).and_then(|chunks| {
                    let s = String::from_utf8(chunks).unwrap();
                    let account_result_json: JsonValue = from_str(&s).unwrap();
                    future::ok::<_, Error>(account_result_json)
                })
            })
        })
    });

    let user_profile = evloop.run(worker).unwrap();

    println!("user_profile returned from twitter: {:?}", user_profile);
    Ok(String::from("Twitter"))
}

