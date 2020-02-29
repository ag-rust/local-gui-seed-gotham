use crate::data::error::Error;
use shared::Counter;

use crate::data::error::serialization_error_response;

use std::sync::{Arc, Mutex};

use hyper::{Body, Response, StatusCode};

use gotham::helpers::http::response::create_response;
use gotham::state::State;

use gotham_derive::StateData;

#[derive(Clone, StateData)]
pub struct CounterState {
    inner: Arc<Mutex<Option<Counter>>>,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }
}

impl CounterState {
    fn compute<F>(&self, arith: F, error: String) -> Result<i32, Error>
    where
        F: Fn(i32, i32) -> Option<i32>,
    {
        if self.initialized() {
            let mut count = self.inner.lock().unwrap();
            match arith(count.as_ref().unwrap().count, 1) {
                Some(result) => {
                    count.as_mut().unwrap().count = result;
                    Ok(result)
                }
                None => Err(Error::new(error)),
            }
        } else {
            Err(Error::new(String::from(
                "Counter has not been initialized.",
            )))
        }
    }

    pub(crate) fn increment(&self) -> Result<i32, Error> {
        self.compute(i32::checked_add, String::from("Reached maximum value."))
    }

    pub(crate) fn decrement(&self) -> Result<i32, Error> {
        self.compute(i32::checked_sub, String::from("Reached minimum value."))
    }

    pub(crate) fn set_default(&self) -> () {
        let mut counter = self.inner.lock().unwrap();
        *counter = Some(Counter::default());
    }

    pub(crate) fn initialized(&self) -> bool {
        let counter = self.inner.lock().unwrap();
        match counter.as_ref() {
            Some(_) => true,
            None => false,
        }
    }

    pub fn response(&self, state: &State) -> Response<Body> {
        let counter = self.inner.lock().unwrap();
        match counter.as_ref() {
            Some(counter) => {
                let counter_serialized = serde_json::to_string(counter);
                match counter_serialized {
                    Ok(body) => {
                        create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, body)
                    }
                    Err(_) => serialization_error_response(&state),
                }
            }
            None => Error::new(String::from("Counter is not initialized!"))
                .into_response(&state, StatusCode::CONFLICT),
        }
    }
}
