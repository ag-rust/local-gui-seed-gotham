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
    inner: Arc<Mutex<Counter>>,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Counter::default())),
        }
    }
}

impl CounterState {
    fn increment(&self) -> Result<i32, Error> {
        let mut count = self.inner.lock().unwrap();
        if count.count < i32::max_value() {
            count.count += 1;
            Ok(count.count)
        } else {
            Err(Error { reason: String::from("Reached maximum value.") })
        }
    }

    fn decrement(&self) -> Result<i32, Error> {
        let mut count = self.inner.lock().unwrap();
        if count.count > i32::min_value() {
            count.count -= 1;
            Ok(count.count)
        } else {
            Err(Error { reason: String::from("Reached minimum value.") })
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


pub fn main() {
    let addr = "127.0.0.1:8080";

    let counter = CounterState::default();
    let middleware = StateMiddleware::new(counter);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);


    let router = build_router(chain, pipelines, |route| {
        route.post("/counter/increment").to(post_counter_increment);
        route.post("/counter/decrement").to(post_counter_decrement);
        route.get("/gui/pkg/*").to_dir(FileOptions::new("./gui/pkg"));
        route.get("/gui/wing.min.css").to_file(FileOptions::new("./gui/wing.min.css"));
        route.get("/").to_file(FileOptions::new("./gui/index.html"));
    });

    gotham::start(addr, router)
}