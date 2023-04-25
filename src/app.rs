use yew::prelude::*;
use yew_router::{prelude::*, route::Route, switch::Permissive};
use crate::pages::{landing::Landing, sign_in::SignIn, dashboard::Dashboard, project_task::ProjectTask, pt_check::PTCheck, profile::Profile, billing::Billing};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/landing"]
    Landing,
    #[to = "/sign-in"]
    SignIn,
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/project-task"]
    ProjectTask,
    #[to = "/pt-check"]
    PTCheck,
    #[to = "/profile"]
    Profile,
    #[to = "/billing"]
    Billing,
    #[to = "/{*:any}"]
    NotFound(Permissive<String>),
}

pub struct App {}

impl Component for App {
    type Message = ();
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
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute| {
                        match switch {
                            AppRoute::Landing => html!{<Landing/>},
                            AppRoute::SignIn => html!{<SignIn/>},
                            AppRoute::Dashboard => html!{<Dashboard/>},
                            AppRoute::ProjectTask => html!{<ProjectTask/>},
                            AppRoute::PTCheck => html!{<PTCheck/>},
                            AppRoute::Profile => html!{<Profile/>},
                            AppRoute::Billing => html!{<Billing/>},
                            AppRoute::NotFound(_) => html!{<div>{"Page not found"}</div>},
                        }
                    })
                </>
            }
        }
    }

