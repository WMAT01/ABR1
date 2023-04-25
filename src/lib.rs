use crate::components::diffs::Diffs;

#![recursion_limit = "1024"]

mod components;
mod services;
mod utils;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    billing::Billing, dashboard::Dashboard, home::Home, landing::Landing, login::Login,
    profile::Profile, register::Register, sign_in::SignIn,
};
use crate::services::auth::Auth;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/login"]
    Login,
    #[to = "/register"]
    Register,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/profile"]
    Profile,
    #[to = "/billing"]
    Billing,
    #[to = "/sign_in"]
    SignIn,
    #[to = "/"]
    Landing,
    // Add the new routes here
    #[to = "/diffs"]
    Diffs,
    #[to = "/text_diff"]
    TextDiff,
    #[to = "/visual_diff"]
    VisualDiff,
    #[to = "/web_diff"]
    WebDiff,    
}

pub struct Model {
    link: ComponentLink<Self>,
    auth: Auth,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            auth: Auth::new(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Login => html!{<Login/>},
                        AppRoute::Register => html!{<Register/>},
                        AppRoute::Dashboard => html!{<Dashboard/>},
                        AppRoute::Profile => html!{<Profile/>},
                        AppRoute::Billing => html!{<Billing/>},
                        AppRoute::SignIn => html!{<SignIn/>},
                        AppRoute::Landing => html!{<Landing/>},
                        // Add the new routes here
                        AppRoute::Diffs => html!{<diffs::Diffs/>},
                        AppRoute::TextDiff => html!{<text_diff::TextDiff/>},
                        AppRoute::VisualDiff => html!{<visual_diff::VisualDiff/>},
                        AppRoute::WebDiff => html!{<web_diff::WebDiff/>},

                    }
                })
            />
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
