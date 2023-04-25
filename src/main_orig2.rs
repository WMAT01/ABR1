match user.database_type {
    DatabaseType::SurrealDB => {
        // Call SurrealDB functions from database.rs
    }
    DatabaseType::SQLite3 => {
        // Call SQLite3 functions from dbsqlite3.rs
    }
}
use yew::prelude::*;
use yew_router::{router::Router, Switch};

mod components {
    pub mod dashboard;
    pub mod landing;
    pub mod sign_in;
}

// Define the app routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/dashboard"]
    Dashboard,
    #[to = "/sign-in"]
    SignIn,
    #[to = "/"]
    Landing,
}

fn main() {
    yew::start_app::<Model>();
}

struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Dashboard => html!{<components::dashboard::Dashboard/>},
                        AppRoute::SignIn => html!{<components::sign_in::SignIn/>},
                        AppRoute::Landing => html!{<components::landing::Landing/>},
                    }
                })
            />
        }
    }
}
