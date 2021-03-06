use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use juniper::{graphql_value, FieldError, IntoFieldError, ScalarValue};
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Debug)]
pub enum CustomError {
  DBConnction(String),
}

#[derive(Debug, Serialize)]
pub struct ApiResponseBody {
  pub success: bool,
  pub errcode: u16,
  pub message: String,
}

impl Display for CustomError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "({})", self)
  }
}

impl<S: ScalarValue> IntoFieldError<S> for CustomError {
  fn into_field_error(self) -> FieldError<S> {
    match self {
      CustomError::DBConnction(msg) => FieldError::new(
        "Internal database error",
        graphql_value!({
            "type": "DATABASE",
            "msg" : msg
        }),
      ),
    }
  }
}

impl ResponseError for CustomError {
  fn status_code(&self) -> StatusCode {
    match self {
      CustomError::DBConnction(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
  fn error_response(&self) -> HttpResponse {
    match self {
      CustomError::DBConnction(msg) => {
        HttpResponse::build(self.status_code()).json(ApiResponseBody {
          success: false,
          errcode: 100,
          message: String::from(msg),
        })
      }
    }
  }
}
