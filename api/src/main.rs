extern crate gotham;
extern crate gotham_derive;
extern crate hyper;
extern crate mime;

use hyper::{Body, Response, StatusCode};
use gotham::handler::assets::FileOptions;
use gotham::router::builder::{DefineSingleRoute, DrawRoutes, build_router};
use shared::Counter;
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
    fn increment(&self) -> i32 {
        let mut count = self.inner.lock().unwrap();
        count.count += 1;
        count.count
    }

    fn decrement(&self) -> i32 {
        let mut count = self.inner.lock().unwrap();
        count.count -= 1;
        count.count
    }
}

fn post_counter_increment(state: State) -> (State, Response<Body>) {
    post_counter(state, CounterState::increment)
}

fn post_counter_decrement(state: State) -> (State, Response<Body>) {
    post_counter(state, CounterState::decrement)
}

fn post_counter<F>(state: State, test: F) -> (State, Response<Body>)
    where F: Fn(&CounterState) -> i32
{
    let response = {
        let counter = CounterState::borrow_from(&state);
        test(counter);
        create_response(
            &state,
            StatusCode::OK,
            mime::APPLICATION_JSON,
            serde_json::to_string(counter.inner.as_ref()).expect("serialized counter"),
        )
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