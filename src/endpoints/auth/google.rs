use std::env;
use std::str::FromStr;

use futures::future;
use futures::{Future, Stream};

use url::form_urlencoded;

use serde_json::from_str;
use serde_json::Value as JsonValue;

use tokio_core::reactor::Core;

use hyper_tls::HttpsConnector;
use hyper::{Uri, Method, Error};
use hyper::client::{Client, Request};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper::header::{Accept, Bearer, Headers, Authorization, ContentType, qitem};

use config::Config;

#[derive(Serialize, Deserialize)]
struct CodeResp {
    access_token: String,
}

pub fn auth(config: &Config, code: String) -> Result<String, String> {

    let client_id = &config.GoogleOauth.client_id;
    let client_secret = &config.GoogleOauth.app_secret;
    let redirect_uri = &config.GoogleOauth.redirect_uri;

    let accessTokenUrl = Uri::from_str("https://accounts.google.com/o/oauth2/token").unwrap();
    let peopleApiUrl = Uri::from_str("https://www.googleapis.com/plus/v1/people/me/openIdConnect").unwrap();

    let mut code_req = Request::new(Method::Post, accessTokenUrl);

    code_req.headers_mut().set(ContentType::form_url_encoded());
    code_req.headers_mut().set(Accept(vec![qitem(
                Mime(TopLevel::Application,
                     SubLevel::Json,
                     vec![(Attr::Charset, Value::Utf8)]))
    ]));

    let code_body = form_urlencoded::Serializer::new(String::new())
        .append_pair("grant_type", "authorization_code")
        .append_pair("code", code.as_str())
        .append_pair("client_id", client_id.as_str())
        .append_pair("redirect_uri", redirect_uri.as_str())
        .append_pair("client_secret", client_secret.as_str())
        .finish();

    code_req.set_body(code_body);

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

            let mut people_req = Request::new(Method::Post, peopleApiUrl);

            people_req.headers_mut().set(Accept(vec![qitem(
                        Mime(TopLevel::Application,
                             SubLevel::Json,
                             vec![(Attr::Charset, Value::Utf8)]))
            ]));

            people_req.headers_mut().set(Authorization(Bearer {
                token: code_result_json.access_token.to_owned()
            }));

            client.request(people_req).and_then(|people_result| {
                println!("Response: {}", people_result.status());
                people_result.body().fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, Error>(v)
                }).and_then(|chunks| {
                    let s = String::from_utf8(chunks).unwrap();
                    let people_result_json: JsonValue = from_str(&s).unwrap();
                    future::ok::<_, Error>(people_result_json)
                })
            })
        })
    });

    let user_profile = evloop.run(worker).unwrap();

    println!("user_profile returned from google: {:?}", user_profile);
    Ok(String::from("GOOGLE"))
}

