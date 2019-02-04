#[macro_use]
extern crate seed;
use seed::prelude::*;


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

fn update(msg: Msg, model: Model) -> Update<Model> {
    match msg {
        Msg::Increment => Render(Model {val: model.val + 1})
    }
}


// View

fn view(_state: seed::App<Msg, Model>, model: &Model) -> El<Msg> {
    button![ 
        simple_ev(Ev::Click, Msg::Increment), 
        format!("Hello, World × {}", model.val) 
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();

}