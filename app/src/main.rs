extern crate web_view;

use api_lib::main as api_server;
use std::thread;
use web_view::*;

fn main() {
    thread::spawn(move || {
        api_server();
    });

    web_view::builder()
        .title("My Project")
        .content(Content::Url(
            "http://localhost:8080/api/v1/assets/index.html",
        ))
        .size(600, 800)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
