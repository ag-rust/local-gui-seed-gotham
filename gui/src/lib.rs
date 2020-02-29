extern crate seed;
extern crate shared;

use seed::browser::service::fetch;
use seed::{prelude::*, *};

use shared::Counter;

const COUNTER_FETCH_URL: &str = "http://localhost:8080/api/v1/counter";
const COUNTER_INIT_URL: &str = "http://localhost:8080/api/v1/counter/init";
const COUNTER_INCREMENT_URL: &str = "http://localhost:8080/api/v1/counter/increment";
const COUNTER_DECREMENT_URL: &str = "http://localhost:8080/api/v1/counter/decrement";
const BACKEND_TERMINATION_URL: &str = "http://localhost:8080/api/v1/terminate";

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

// ---- -----
//  Window Events
// ---- -----

fn window_events(_model: &Model) -> Vec<EventHandler<Msg>> {
    vec![ev(Ev::BeforeUnload, Msg::OnClose)]
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
    OnClose(web_sys::Event),
    BackendTerminated(fetch::ResponseDataResult<String>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => {
            orders.skip().perform_cmd(increment_counter());
        }

        Msg::Decrement => {
            orders.skip().perform_cmd(decrement_counter());
        }

        Msg::CounterFetched(Ok(counter)) => model.counter = counter,

        Msg::CounterFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching counter failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }

        Msg::OnClose(_event) => {
            log!("closing");
            orders.skip().perform_cmd(terminate_backend());
        }

        Msg::BackendTerminated(_) => {} // no need to handle it, the application is closed here
    };
}

async fn terminate_backend() -> Result<Msg, Msg> {
    Request::new(BACKEND_TERMINATION_URL)
        .method(Method::Post)
        .fetch_json_data(Msg::BackendTerminated)
        .await
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

async fn modify_counter(endpoint: &'static str) -> Result<Msg, Msg> {
    Request::new(endpoint)
        .method(Method::Post)
        .fetch_json_data(Msg::CounterFetched)
        .await
}

async fn decrement_counter() -> Result<Msg, Msg> {
    modify_counter(COUNTER_DECREMENT_URL).await
}

async fn increment_counter() -> Result<Msg, Msg> {
    modify_counter(COUNTER_INCREMENT_URL).await
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
                    button![ev(Ev::Click, |_| Msg::Increment), "+"],
                    button![ev(Ev::Click, |_| Msg::Decrement), "-"],
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
        .window_events(window_events)
        .build_and_start();
}
