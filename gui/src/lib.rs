extern crate seed;
extern crate shared;

use seed::browser::service::fetch;
use seed::{prelude::*, *};

use shared::Counter;

const API_URL: &str = "http://localhost:8080/api/v1";
const COUNTER_FETCH_URL: &str = "http://localhost:8080/api/v1/counter";
const COUNTER_INIT_URL: &str = "http://localhost:8080/api/v1/counter/init";

struct Model {
    counter: Counter,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: Counter::default(),
        }
    }
}
// ----- -----
// After Mount (Initialization)
// ----- -----

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.perform_cmd(init_counter());
    AfterMount::default()
}

// ------ -----
//    Update
// ----- -----

enum Msg {
    Increment,
    Decrement,
    CounterFetched(fetch::ResponseDataResult<Counter>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter.count += 1,
        Msg::Decrement => model.counter.count -= 1,

        Msg::CounterFetched(Ok(counter)) => model.counter = counter,

        Msg::CounterFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching counter failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }
    };
}

async fn fetch_counter() -> Result<Msg, Msg> {
    Request::new(COUNTER_FETCH_URL)
        .fetch_json_data(Msg::CounterFetched)
        .await
}

async fn init_counter() -> Result<Msg, Msg> {
    Request::new(COUNTER_INIT_URL)
        .method(Method::Post)
        .fetch_json_data(Msg::CounterFetched)
        .await
}

// ------ -----
//     View
// ------ -----

fn view(model: &Model) -> impl View<Msg> {
    div![
        div![
            attrs! {At::Class => "row"},
            div![
                attrs! {At::Class => "col"; At::Class => "center"; },
                h1!["Let's count"],
            ],
        ],
        div![
            attrs! {At::Class => "row"},
            div![
                attrs! {At::Class => "card"; At::Class => "col" },
                h3![
                    attrs![At::Class => "card-header" ],
                    format!("Current count is {}", model.counter.count)
                ],
                p![
                    attrs![At::Class => "card-body" ],
                    //button![simple_ev(Ev::Click, Msg::Increment), "+"],
                    //button![simple_ev(Ev::Click, Msg::Decrement), "-"],
                ],
            ],
        ],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
