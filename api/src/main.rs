extern crate gotham;
extern crate gotham_derive;
extern crate hyper;
extern crate mime;

mod api;
mod data;

use data::counter::CounterState;

use crate::api::counter::{
    get_counter, post_counter_decrement, post_counter_increment, post_counter_init,
};
use gotham::handler::assets::FileOptions;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::{single::single_pipeline, single_middleware};
use gotham::router::builder::{build_router, DefineSingleRoute, DrawRoutes};
use gotham::router::Router;

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
        route
            .get("/gui/pkg/*")
            .to_dir(FileOptions::new("./gui/pkg"));
        route
            .get("/gui/wing.min.css")
            .to_file(FileOptions::new("./gui/wing.min.css"));
        route.get("/").to_file(FileOptions::new("./gui/index.html"));
    });

    gotham::start(addr, router)
}
