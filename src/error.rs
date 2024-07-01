use actix_web::http::StatusCode;
use actix_web::{error, http::header::ContentType, HttpResponse};
use argon2;
use mail_send;
use r2d2;
use rusqlite;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum UserError {
   #[error("Expected true, false, 1, or 0")]
   BadBool,
   #[error("Index out of range.")]
   IndexOutOfRange,
   #[error("An internal error occurred. Please try again later.")]
   InternalError(String),
   #[error("Invalid email.")]
   InvalidEmail,
   #[error("Not authorized.")]
   NotAuthorized,
   #[error("Requested resource was not found.")]
   NotFound,
   #[error("Unexpected input.")]
   UnexpectedInput,
}

impl error::ResponseError for UserError {
   fn error_response(&self) -> HttpResponse {
      HttpResponse::build(self.status_code())
         .insert_header(ContentType::html())
         .body(self.to_string())
   }

   fn status_code(&self) -> StatusCode {
      match *self {
         UserError::BadBool => StatusCode::BAD_REQUEST,
         UserError::IndexOutOfRange => StatusCode::BAD_REQUEST,
         UserError::InternalError(_) => {
            StatusCode::INTERNAL_SERVER_ERROR
         }
         UserError::InvalidEmail => StatusCode::BAD_REQUEST,
         UserError::NotAuthorized => StatusCode::UNAUTHORIZED,
         UserError::NotFound => StatusCode::NOT_FOUND,
         UserError::UnexpectedInput => StatusCode::BAD_REQUEST,
      }
   }
}

impl From<actix_web::http::header::ToStrError> for UserError {
   fn from(error: actix_web::http::header::ToStrError) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<std::array::TryFromSliceError> for UserError {
   fn from(error: std::array::TryFromSliceError) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<aes_gcm::aes::cipher::InvalidLength> for UserError {
   fn from(error: aes_gcm::aes::cipher::InvalidLength) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<aes_gcm::Error> for UserError {
   fn from(error: aes_gcm::Error) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<std::string::FromUtf8Error> for UserError {
   fn from(error: std::string::FromUtf8Error) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<base64::DecodeError> for UserError {
   fn from(error: base64::DecodeError) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<tera::Error> for UserError {
   fn from(error: tera::Error) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<actix_session::SessionGetError> for UserError {
   fn from(error: actix_session::SessionGetError) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<actix_session::SessionInsertError> for UserError {
   fn from(error: actix_session::SessionInsertError) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<r2d2::Error> for UserError {
   fn from(error: r2d2::Error) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<argon2::password_hash::Error> for UserError {
   fn from(error: argon2::password_hash::Error) -> Self {
      UserError::InternalError(error.to_string())
   }
}

impl From<rusqlite::Error> for UserError {
   fn from(error: rusqlite::Error) -> Self {
      match error {
         rusqlite::Error::QueryReturnedNoRows => UserError::NotFound,
         _ => UserError::InternalError(error.to_string()),
      }
   }
}

impl From<mail_send::Error> for UserError {
   fn from(error: mail_send::Error) -> Self {
      match error {
         mail_send::Error::UnexpectedReply(_) => {
            UserError::InvalidEmail
         }
         _ => UserError::InternalError(error.to_string()),
      }
   }
}
