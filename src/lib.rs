#[macro_use]
extern crate seed;
use seed::prelude::*;


// Model

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

fn update(msg: Msg, model: &mut Model) -> Update<Msg> {
    match msg {
        Msg::Increment => model.val += 1,
    }
    Render.into()
}


// View

fn view(model: &Model) -> Vec<El<Msg>> {
    vec![
        button![
            simple_ev(Ev::Click, Msg::Increment),
            format!("Hello, World × {}", model.val)
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();

}