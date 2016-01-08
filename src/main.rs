extern crate rustc_serialize;
#[macro_use] extern crate log;
#[macro_use] extern crate nickel;

mod token_info;

use nickel::{Nickel, MediaType, HttpRouter, QueryString};
use nickel::status::StatusCode::BadRequest;
use rustc_serialize::json;
use token_info::TokenInfo;

static ACCESS_TOKEN_INVALID: &'static str = "Access token invalid.";

fn invalid_request<S: Into<String>>(err: S) -> String {
    format!("{{\"error\":\"invalid_request\",\"error_description\":\"{}\"}}", err.into())
}

fn main() {
    let mut server = Nickel::new();

    server.get("/oauth2/tokeninfo", middleware! { |req, mut res|
        res.set(MediaType::Json);
        let token = match req.query().get("access_token") {
            Some(token) => token.to_string(),
            None => {
                res.set(BadRequest);
                return res.send(invalid_request(ACCESS_TOKEN_INVALID))
            }
        };

        let token_info = match TokenInfo::from_query_param(&token) {
            Ok(token_info) => token_info,
            Err(err) => {
                res.set(BadRequest);
                return res.send(invalid_request(err));
            }
        };

        json::encode(&token_info).unwrap()
    });

    server.listen("0.0.0.0:8080");
}
