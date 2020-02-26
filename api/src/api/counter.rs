use crate::data::counter::CounterState;
use crate::data::error::Error;
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};

pub fn post_counter_increment(state: State) -> (State, Response<Body>) {
    post_counter(state, CounterState::increment)
}

pub fn post_counter_decrement(state: State) -> (State, Response<Body>) {
    post_counter(state, CounterState::decrement)
}

fn post_counter<F>(state: State, count: F) -> (State, Response<Body>)
where
    F: Fn(&CounterState) -> Result<i32, Error>,
{
    let response = {
        let counter = CounterState::borrow_from(&state);
        match count(counter) {
            Ok(_) => counter.response(&state),
            Err(e) => e.into_response(&state, StatusCode::CONFLICT),
        }
    };
    (state, response)
}

pub fn get_counter(state: State) -> (State, Response<Body>) {
    let response = {
        let counter = CounterState::borrow_from(&state);
        counter.response(&state)
    };
    (state, response)
}

pub fn post_counter_init(state: State) -> (State, Response<Body>) {
    let response = {
        let counter = CounterState::borrow_from(&state);
        if counter.initialized() {
            Error::new(String::from("Counter already initialized."))
                .into_response(&state, StatusCode::CONFLICT)
        } else {
            counter.set_default();
            counter.response(&state)
        }
    };
    (state, response)
}
