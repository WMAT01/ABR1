use yew::prelude::*;
use yew_styles::layouts::{Container, Item};
use yew_styles::styles::{Palette, Size, Style};
use yew_styles::button::{Button, ButtonType};
use yew_router::prelude::*;

pub struct Landing {}

pub enum Msg {}

impl Component for Landing {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Container direction="column" wrap="nowrap" class="landing">
                    <Item layouts=vec!(("is-12", "is-flex", "is-justify-content-center"))>
                        <img src="path/to/logo.png" alt="Logo" />
                    </Item>
                    <Item layouts=vec!(("is-12", "is-flex", "is-justify-content-center"))>
                        <h1 class="title">{ "ABR" }</h1>
                    </Item>
                    <Item layouts=vec!(("is-12", "is-flex", "is-justify-content-center"))>
                        <h3 class="tagline">{ "Empowering Insights, Enhancing Expertise, Edging-out the Competition" }</h3>
                    </Item>
                    <Item layouts=vec!(("is-12", "is-flex", "is-justify-content-center"))>
                        <Button
                            button_type=ButtonType::Outlined
                            style=Style::Dark
                            palette=Palette::Danger
                            size=Size::Medium
                            onclick=Callback::noop()
                        >
                            <RouterAnchor<AppRoute> route=AppRoute::SignIn>
                                { "Sign In" }
                            </RouterAnchor<AppRoute>>
                        </Button>
                    </Item>
                </Container>
            </>
        }
    }
}
