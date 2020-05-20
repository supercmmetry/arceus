use std::env;
use std::result::Result;
use jsonwebtoken::{Header, Algorithm, EncodingKey, Validation, DecodingKey};
use rocket::request::{FromRequest, Outcome};
use rocket::outcome::IntoOutcome;
use rocket::http::{ContentType, Status};
use rocket::{Request, response, Response};

use crate::utils::errors::Error;
use crate::utils::errors::ErrorCode;
use rocket::response::Responder;
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    exp: usize
}

impl Claims {
    pub fn new(username: String) -> Claims {
        Claims {
            username,
            exp: 10000000000
        }
    }

    pub fn jwt(&self) -> Result<String, Error> {
        let mut header = Header::default();
        header.alg = Algorithm::HS512;
        header.kid = Some(env::var("JWT_SIGNING_KEY").unwrap());
        let key = env::var("JWT_PASSWORD").unwrap();


        match jsonwebtoken::encode(&header, self, &EncodingKey::from_secret(key.as_bytes())) {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::new(ErrorCode::AuthTokenCreationFailed))
        }
    }

    pub fn from(token: String) -> Result<Claims, Error> {
        let key = env::var("JWT_PASSWORD").unwrap();
        match jsonwebtoken::decode::<Claims>(&token, &DecodingKey::from_secret(key.as_bytes()),
                                                        &Validation::new(Algorithm::HS512)) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(Error::new(ErrorCode::MalformedAuthToken))
        }
    }
}

pub struct ClaimResult(Result<Claims, Error>);

impl<'a, 'r> FromRequest<'a, 'r> for ClaimResult {
    type Error = Error;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let values: Vec<_> = request.headers().get("Authorization").collect();
        if values.len() > 1 {
            return Outcome::Success(ClaimResult(Err(Error::new(ErrorCode::MultipleAuthToken))));
        } else if values.len() == 0 {
            return Outcome::Success(ClaimResult(Err(Error::new(ErrorCode::NoAuthToken))));
        }

        let token = values[0].to_string();

        Outcome::Success(ClaimResult(Claims::from(token)))
    }
}

impl<'r> Responder<'r> for ClaimResult {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        match self {
            ClaimResult(Ok(claims)) => Response::build().header(ContentType::JSON)
                .sized_body(Cursor::new(claims.username)).ok(),
            ClaimResult(Err(e)) => e.respond_to(request),
        }
    }
}


