
#![recursion_limit = "1024"]

mod app;
mod components;
mod state;

use yew::prelude::*;
use yew_router::{router::Router, Switch};

use wasm_bindgen::prelude::*;


pub enum AppRoute {
    Landing,
    SignIn,
    Dashboard,
    ProjectTask,
    PTCheck,
    Profile,
    Billing,
}

impl Switch for AppRoute {
    type YewRouterState = ();

    fn from_route_part<It>(parts: It) -> Option<Self>
    where
        It: Iterator<Item = String> + Clone,
    {
        let mut parts = parts.clone();
        match parts.next()?.as_str() {
            "signin" => Some(AppRoute::SignIn),
            "dashboard" => Some(AppRoute::Dashboard),
            "project_task" => Some(AppRoute::ProjectTask),
            "pt_check" => Some(AppRoute::PTCheck),
            "profile" => Some(AppRoute::Profile),
            "billing" => Some(AppRoute::Billing),
            _ => Some(AppRoute::Landing),
        }
    }

    fn build_route_section<It>(&self, parts: &mut It) -> String
    where
        It: Iterator<Item = String> + Clone,
    {
        match self {
            AppRoute::Landing => "/".to_string(),
            AppRoute::SignIn => "/signin".to_string(),
            AppRoute::Dashboard => "/dashboard".to_string(),
            AppRoute::ProjectTask => "/project_task".to_string(),
            AppRoute::PTCheck => "/pt_check".to_string(),
            AppRoute::Profile => "/profile".to_string(),
            AppRoute::Billing => "/billing".to_string(),
        }
    }
}

fn main() {
    yew::start_app::<app::App>();

/*    match user.database_type {
    DatabaseType::SurrealDB => {
        // Call SurrealDB functions from database.rs
    }
    DatabaseType::SQLite3 => {
        // Call SQLite3 functions from dbsqlite3.rs
    }
}
#![recursion_limit = "1024"]
*/


}
