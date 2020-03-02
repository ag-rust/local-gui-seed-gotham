pub mod assets;
pub mod counter;

use gotham::state::State;
use hyper::{Body, Response};

pub fn terminate(_state: State) -> (State, Response<Body>) {
    std::process::exit(0);
}
