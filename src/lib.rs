#[macro_use]
extern crate seed;
use seed::prelude::*;
use wasm_bindgen::prelude::*;


// Model

#[derive(Clone)]
struct Model {
    pub val: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}


// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::Increment => Model {val: model.val + 1}
    }
}


// View

fn view(app: seed::App<Msg, Model>, model: Model) -> El<Msg> {
    button![ 
        simple_ev("click", Msg::Increment), 
        format!("Hello, World × {}", model.val) 
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::run(Model::default(), update, view, "main", None);
}