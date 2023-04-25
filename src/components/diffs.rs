use yew::prelude::*;

use crate::components::text_diff::TextDiff;
use crate::components::visual_diff::VisualDiff;
use crate::components::web_diff::WebDiff;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Diffs {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub orientation: Orientation,
    pub text_diff_content: String,
    pub visual_diff_content: String,
    pub web_diff_content: String,
}

pub enum Msg {
    ToggleOrientation,
}

impl Component for Diffs {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Diffs { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleOrientation => {
                self.props.orientation = match self.props.orientation {
                    Orientation::Horizontal => Orientation::Vertical,
                    Orientation::Vertical => Orientation::Horizontal,
                };
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let orientation_class = match self.props.orientation {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        };

        html! {
            <div class=("diff-container", orientation_class)>
                <TextDiff content=&self.props.text_diff_content />
                <VisualDiff content=&self.props.visual_diff_content />
                <WebDiff content=&self.props.web_diff_content />
                <button onclick=self.link.callback(|_| Msg::ToggleOrientation)>
                    { "Toggle Orientation" }
                </button>
            </div>
        }
    }
}
