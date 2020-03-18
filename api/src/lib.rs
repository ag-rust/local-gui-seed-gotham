extern crate gotham;
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate rust_embed;
extern crate serde;

mod api;
mod data;

use data::counter::CounterState;

use crate::api::{
    assets::{get_asset, PathExtractor},
    counter::{get_counter, post_counter_decrement, post_counter_increment, post_counter_init},
    terminate,
};
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::{single::single_pipeline, single_middleware};
use gotham::router::builder::{build_router, DefineSingleRoute, DrawRoutes};

pub fn main() {
    let addr = "127.0.0.1:8080";

    let counter = CounterState::default();
    let middleware = StateMiddleware::new(counter);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    let router = build_router(chain, pipelines, |route| {
        route.post("api/v1/terminate").to(terminate);
        route.get("api/v1/counter").to(get_counter);
        route.post("api/v1/counter/init").to(post_counter_init);
        route
            .post("api/v1/counter/increment")
            .to(post_counter_increment);
        route
            .post("api/v1/counter/decrement")
            .to(post_counter_decrement);
        route
            .get("api/v1/assets/*")
            .with_path_extractor::<PathExtractor>()
            .to(get_asset);
    });

    gotham::start(addr, router)
}
