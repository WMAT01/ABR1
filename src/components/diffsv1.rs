use seed::{prelude::*, *};
use wasm_bindgen::JsCast;

pub(crate) fn view() -> Node<Msg> {
    div![
        C!["diffs-container"],
        iframe![
            C!["text-diff"],
            attrs! {
                At::Src => "/path/to/text_diff.html",
            }
        ],
        iframe![
            C!["visual-diff"],
            attrs! {
                At::Src => "/path/to/visual_diff.html",
            }
        ],
        iframe![
            C!["web-diff"],
            attrs! {
                At::Src => "/path/to/web_diff.html",
            }
        ],
        button![
            C!["toggle-orientation"],
            "Toggle Orientation",
            ev(Ev::Click, |_| Msg::ToggleOrientation)
        ],
    ]
}

#[derive(Clone, Debug)]
pub enum Msg {
    ToggleOrientation,
}

pub(crate) struct Model {
    pub(crate) horizontal: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self { horizontal: true }
    }
}

pub(crate) fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleOrientation => {
            model.horizontal = !model.horizontal;
            let iframes = seed::document().get_elements_by_class_name("diffs-container");
            for i in 0..iframes.length() {
                let iframe = iframes
                    .get(i)
                    .expect("iframe not found")
                    .dyn_into::<web_sys::HtmlIFrameElement>()
                    .expect("iframe not found");
                if model.horizontal {
                    iframe.set_class_name("horizontal");
                } else {
                    iframe.set_class_name("vertical");
                }
            }
        }
    }
}
