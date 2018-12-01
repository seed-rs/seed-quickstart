use wasm_bindgen::prelude::*;

// This prelude is the equivalent of the following imports:
// use rebar::dom_types::{El, Style, Attrs, Tag, Event, Events, UpdateEl};
// use rebar::vdom::App;
use crate::prelude::*;


// Model

#[derive(Clone, Debug)] // todo
struct Model {
    pub val: i32,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self {
            val: 0,
        }
    }
}

// Update

enum Msg {
    Increment,
}

//fn update(msg: &Msg, model: Rc<Model>) -> Model {
fn update(msg: &Msg, model: Rc<RefCell<Model>>) -> Model {
    // todo msg probably doesn't need to be a ref.
//    let model2 = model.clone(); // todo deal with this.
    match msg {
        &Msg::Increment => {
//            Model {clicks: model.clicks + 1, ..model.unwrap()}
            Model {clicks: model.borrow().clicks + 1, what_we_count: String::from("test")}
        },
    }
}

// View

// Top-level component we pass to the virtual dom. Must accept the model as its only argument.
fn comp(model: &Model) -> El<Msg> {
    !div[]]
}

#[wasm_bindgen]
pub fn render() -> Result<(), JsValue> {
    let model = Model::default();

    let mut app = App::new(model, update, comp, "main");
    app.mount()
}