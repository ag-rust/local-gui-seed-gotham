extern crate seed;
extern crate shared;

use seed::{*, prelude::*};

use shared::Counter;

struct Model {
    counter: Counter,
}

impl Default for Model {
    fn default() -> Self {
        Self{counter: Counter::default()}
    }
}

#[derive(Clone)]
enum Msg {
    Increment,
    Decrement
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
        Msg::Decrement => model.counter -= 1,
    };
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        div![ attrs!{At::Class => "row"},
            div![ attrs!{At::Class => "col"; At::Class => "center"; },
                h1![ "Let's count" ],
            ],
        ],
        div![ attrs!{At::Class => "row"},
            div![ attrs!{At::Class => "card"; At::Class => "col" },
                    h3![ attrs![At::Class => "card-header" ], format!("Current count is {}", model.counter.get_count()) ],
                    p![ attrs![At::Class => "card-body" ],
                        button![ simple_ev(Ev::Click, Msg::Increment), "+" ],
                        button![ simple_ev(Ev::Click, Msg::Decrement), "-" ],
                    ],
            ],
        ],
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .build_and_start();
}