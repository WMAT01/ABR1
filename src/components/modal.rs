use yew::prelude::*;

pub struct Modal {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_open: bool,
    pub on_close: Callback<()>,
    pub children: Children,
}

impl Component for Modal {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        if self.props.is_open {
            html! {
                <div class="modal">
                    <div class="modal-content">
                        { for self.props.children.iter() }
                        <button onclick=self.props.on_close.clone()>{ "Close" }</button>
                    </div>
                </div>
            }
        } else {
            html! {}
        }
    }
}
