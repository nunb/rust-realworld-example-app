use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use core::types::user::CurrentUser;
use settings::Settings;

use jwt::{Algorithm, decode};

impl<'a, 'r> FromRequest<'a, 'r> for CurrentUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<CurrentUser, ()> {
        let access_token = request.headers().get_one("Authorization");
        let settings = request.guard::<State<Settings>>()?;
        let secret = settings.auth.secret.clone();

        match access_token {
            Some(token) if token.len() > 7 => {
                let token = &token[6..]; // "Token JWTToken"
                println!("token {:?}", token);
                let decoded = decode(&token.to_string(), &secret, Algorithm::HS256);
                println!("{:?}", decoded);
                match decoded {
                    Ok((_header, payload)) => match payload.get("sub") {
                        Some(sub) => Outcome::Success(CurrentUser{ username: String::from(sub.as_str().unwrap_or(""))}),
                        None => Outcome::Failure((Status::Unauthorized, ())),
                    }
                    Err(_err) => Outcome::Failure((Status::Unauthorized, ())),
                }
            }
            _ => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
