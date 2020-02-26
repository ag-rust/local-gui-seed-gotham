use shared::Error as SError;

use hyper::{Body, Response, StatusCode};

use gotham::helpers::http::response::create_response;
use gotham::state::State;

pub struct Error(SError);

impl Error {
    pub fn new(msg: String) -> Error {
        Error(SError { reason: msg })
    }

    pub fn into_response(self, state: &State, status_code: StatusCode) -> Response<Body> {
        create_response(
            &state,
            status_code,
            mime::APPLICATION_JSON,
            serde_json::to_string(&self.0.reason).expect("Failed to serialize error message."),
        )
    }
}

pub fn serialization_error_response(state: &State) -> Response<Body> {
    Error::new(String::from("Failed to serialize struct!"))
        .into_response(state, StatusCode::INTERNAL_SERVER_ERROR)
}
