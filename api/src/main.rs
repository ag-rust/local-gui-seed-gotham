extern crate gotham;
extern crate gotham_derive;
extern crate hyper;
extern crate mime;

use hyper::{Body, Response, StatusCode};
use gotham::handler::assets::FileOptions;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes, build_router};
use shared::{Counter, Error};
use std::sync::{Mutex, Arc};
use gotham::state::{State, FromState};
use gotham::helpers::http::response::create_response;
use gotham_derive::StateData;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single_middleware;
use gotham::pipeline::single::single_pipeline;

#[derive(Clone, StateData)]
struct CounterState {
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
    fn increment(&self) -> Result<i32, Error> {
        let mut count = self.inner.lock().unwrap();
        match count.as_mut() {
            Some(counter) => {
                if counter.count < i32::max_value() {
                    counter.count += 1;
                    Ok(counter.count)
                } else {
                    Err(Error { reason: String::from("Reached maximum value.") })
                }
            },
            None => Err(Error { reason: String::from("Counter has not been initialized") }),
        }
    }

    fn decrement(&self) -> Result<i32, Error> {
        let mut count = self.inner.lock().unwrap();
        match count.as_mut() {
            Some(counter) => {
                if counter.count > i32::min_value() {
                    counter.count -= 1;
                    Ok(counter.count)
                } else {
                    Err(Error { reason: String::from("Reached minimum value.") })
                }
            },
            None => Err(Error { reason: String::from("Counter has not been initialized") }),
        }
    }

    fn set_default(&self) -> () {
        let mut counter = self.inner.lock().unwrap();
        *counter = Some(Counter::default());
    }

    fn initialized(&self) -> bool {
        let counter = self.inner.lock().unwrap();
        match counter.as_ref() {
            Some(_) => true,
            None => false,
        }
    }
}

fn post_counter_increment(state: State) -> (State, Response<Body>) {
    post_counter(state, CounterState::increment)
}

fn post_counter_decrement(state: State) -> (State, Response<Body>) {
    post_counter(state, CounterState::decrement)
}

fn post_counter<F>(state: State, count: F) -> (State, Response<Body>)
    where F: Fn(&CounterState) -> Result<i32, Error>
{
    let response = {
        let counter = CounterState::borrow_from(&state);
        match count(counter) {
            Ok(_) => {
                create_response(
                    &state,
                    StatusCode::OK,
                    mime::APPLICATION_JSON,
                    serde_json::to_string(counter.inner.as_ref()).expect("serialized counter"),
                )
            },
            Err(e) => {
                create_response(
                    &state,
                    StatusCode::CONFLICT,
                    mime::APPLICATION_JSON,
                    serde_json::to_string(&e).expect("serialized error"),
                )
            }
        }
    };
    (state, response)
}

fn get_counter(state: State) -> (State, Response<Body>) {
    let response = {
        let counter = CounterState::borrow_from(&state);
        if counter.initialized() {
            create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_string(counter.inner.as_ref()).expect("serialized counter"),
            )
        } else {
            create_response(
                &state,
                StatusCode::CONFLICT,
                mime::APPLICATION_JSON,
                serde_json::to_string(&Error { reason: String::from("Counter has not been initialized") } ).expect("serialized error"),
            )
        }
    };
    (state, response)
}

fn post_counter_init(state: State) -> (State, Response<Body>) {
    let response = {
        let counter = CounterState::borrow_from(&state);
        if counter.initialized() {
            create_response(
                &state,
                StatusCode::CONFLICT,
                mime::APPLICATION_JSON,
                serde_json::to_string(&Error { reason: String::from("Counter already initialized.") }).expect("serialized error"),
            )
        } else {
            counter.set_default();
            create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_string(counter.inner.as_ref()).expect("serialized counter"),
            )
        }
    };
    (state, response)
}


pub fn main() {
    let addr = "127.0.0.1:8080";

    let counter = CounterState::default();
    let middleware = StateMiddleware::new(counter);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);


    let router = build_router(chain, pipelines, |route| {
        route.get("/counter").to(get_counter);
        route.post("/counter/init").to(post_counter_init);
        route.post("/counter/increment").to(post_counter_increment);
        route.post("/counter/decrement").to(post_counter_decrement);
        route.get("/gui/pkg/*").to_dir(FileOptions::new("./gui/pkg"));
        route.get("/gui/wing.min.css").to_file(FileOptions::new("./gui/wing.min.css"));
        route.get("/").to_file(FileOptions::new("./gui/index.html"));
    });

    gotham::start(addr, router)
}