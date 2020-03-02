use gotham::helpers::http::response::create_response;
use gotham::state::FromState;
use gotham::state::State;
use hyper::{Body, Response, StatusCode};
use mime_guess;
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::borrow::Cow;

use gotham_derive::{StateData, StaticResponseExtender};

#[derive(RustEmbed)]
#[folder = "../gui/assets"]
struct Assets;

#[derive(Debug, Deserialize, StateData, StaticResponseExtender)]
pub struct PathExtractor {
    #[serde(rename = "*")]
    parts: Vec<String>,
}

pub fn get_asset(state: State) -> (State, Response<Body>) {
    let path = PathExtractor::borrow_from(&state);
    let path = &path.parts.join("/");
    let content = Assets::get(path).unwrap();

    let response = match content {
        Cow::Borrowed(content) => create_response(
            &state,
            StatusCode::OK,
            mime_guess::from_path(path).first_or_text_plain(),
            content,
        ),
        Cow::Owned(content) => create_response(
            &state,
            StatusCode::OK,
            mime_guess::from_path(path).first_or_text_plain(),
            content,
        ),
    };
    (state, response)
}
