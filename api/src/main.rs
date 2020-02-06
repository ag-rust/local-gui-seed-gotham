use gotham::handler::assets::FileOptions;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};

pub fn main() {
    let addr = "127.0.0.1:8080";

    let router = build_simple_router(|route| {
        route.get("gui/pkg/*").to_dir(FileOptions::new("./gui/pkg"));
        route.get("/").to_file(FileOptions::new("./gui/index.html"));
    });

    gotham::start(addr, router)
}