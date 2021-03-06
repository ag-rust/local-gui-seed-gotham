extern crate seed;
extern crate shared;

use seed::browser::service::fetch;
use seed::{prelude::*, *};

use shared::Counter;

const _COUNTER_FETCH_URL: &str = "http://localhost:8080/api/v1/counter";
const COUNTER_INIT_URL: &str = "http://localhost:8080/api/v1/counter/init";
const COUNTER_INCREMENT_URL: &str = "http://localhost:8080/api/v1/counter/increment";
const COUNTER_DECREMENT_URL: &str = "http://localhost:8080/api/v1/counter/decrement";
const BACKEND_TERMINATION_URL: &str = "http://localhost:8080/api/v1/terminate";
const README_FETCH_URL: &str = "http://localhost:8080/api/v1/assets/README.md";

enum Window {
    Counter,
    About,
}

struct Model {
    counter: Counter,
    window: Window,
    readme: String,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            counter: Counter::default(),
            window: Window::Counter,
            readme: "".to_owned(),
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
    orders.perform_cmd(fetch_readme());
    AfterMount::default()
}

/// ----- ------
/// Routing (Different Windows/Views)
/// ------ ------

fn routes(url: Url) -> Option<Msg> {
    if url.path.is_empty() {
        return Some(Msg::ChangeWindow(Window::Counter));
    }

    Some(match url.path[0].as_ref() {
        "counter" => Msg::ChangeWindow(Window::Counter),
        "about" => Msg::ChangeWindow(Window::About),
        _ => Msg::ChangeWindow(Window::Counter),
    })
}

// ------ -----
//    Update
// ----- -----

enum Msg {
    Increment,
    Decrement,
    CounterFetched(fetch::ResponseDataResult<Counter>),
    ReadmeFetched(fetch::ResponseDataResult<String>),
    ChangeWindow(Window),
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

        Msg::ReadmeFetched(Ok(content)) => {
            let mut options = pulldown_cmark::Options::empty();
            options.insert(pulldown_cmark::Options::ENABLE_TABLES);
            options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
            options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
            options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
            let parser = pulldown_cmark::Parser::new_ext(&content, options);

            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);
            model.readme = html;
            orders.skip(); // no need to redraw here, it's not displayed yet
        }

        Msg::ReadmeFetched(Err(fail_reason)) => {
            error!(format!(
                "Fetch error - Fetching readme failed - {:#?}",
                fail_reason
            ));
            orders.skip();
        }

        Msg::ChangeWindow(window) => model.window = window,

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

async fn _fetch_counter() -> Result<Msg, Msg> {
    Request::new(_COUNTER_FETCH_URL)
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

async fn fetch_readme() -> Result<Msg, Msg> {
    Request::new(README_FETCH_URL)
        .method(Method::Get)
        .fetch_string_data(Msg::ReadmeFetched)
        .await
}

// ------ -----
//     Views
// ------ -----

fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            attrs! {At::Class => "nav"},
            h5!["Local Seed + Gotham GUI", attrs! {At::Class => "nav-logo"}],
            a![
                "Counter",
                attrs! {At::Href => "/counter", At::Class => "nav-item"}
            ],
            a![
                "About",
                attrs! {At::Href => "/about", At::Class => "nav-item"}
            ],
        ],
        match model.window {
            Window::Counter => view_counter(model),
            Window::About => view_about(model),
        },
    ]
}

fn view_about(model: &Model) -> Vec<Node<Msg>> {
    vec![div![attrs! {At::Class => "col",}, raw!(&model.readme),]]
}

fn view_counter(model: &Model) -> Vec<Node<Msg>> {
    vec![
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
        .routes(routes)
        .build_and_start();
}
