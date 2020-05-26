use rocket::http::{Status, ContentType};
use rocket::response::Responder;
use rocket::{Request, response, Response};
use serde_json;
use std::io::Cursor;

#[derive(Debug, Serialize, Copy, Clone)]
pub enum ErrorCode {
    MalformedAuthToken,
    NoAuthToken,
    MultipleAuthToken,
    AuthTokenCreationFailed,
    DatabaseError
}

#[derive(Debug, Serialize, Clone)]
pub struct Error {
    error: ErrorCode,
    message: String
}

impl Error {
    pub fn new(code: ErrorCode) -> Error {
        Error {
            error: code,
            message: Error::map_to_err(&code)
        }
    }

    pub fn custom(code: ErrorCode, msg: String) -> Error {
        Error {
            error: code,
            message: msg
        }
    }

    fn map_to_err(code: &ErrorCode) -> String {
        let str = match code {
            ErrorCode::MalformedAuthToken => "malformed auth token",
            ErrorCode::AuthTokenCreationFailed => "auth token creation failed",
            ErrorCode::NoAuthToken => "no auth token was found",
            ErrorCode::MultipleAuthToken => "multiple auth tokens found",
            ErrorCode::DatabaseError => "database error occured"
        };

        str.to_string()
    }

    fn map_to_status(code: &ErrorCode) -> Status {
        match code {
            ErrorCode::MalformedAuthToken => Status::Forbidden,
            ErrorCode::AuthTokenCreationFailed => Status::InternalServerError,
            ErrorCode::NoAuthToken => Status::Forbidden,
            ErrorCode::MultipleAuthToken => Status::Forbidden,
            ErrorCode::DatabaseError => Status::InternalServerError
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Response::build()
            .status(Error::map_to_status(&self.error))
            .header(ContentType::JSON)
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}