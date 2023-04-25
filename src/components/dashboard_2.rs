match user.database_type {
    DatabaseType::SurrealDB => {
        // Call SurrealDB functions from database.rs
    }
    DatabaseType::SQLite3 => {
        // Call SQLite3 functions from dbsqlite3.rs
    }
}
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct PTCheck {
    id: Uuid,
    timestamp: DateTime<Utc>,
    compressed_response: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectTask {
    pub id: String,
    pub name: String,
    pub url: String,
    pub pt_checks: Vec<PTCheck>,
    pub keywords: Vec<String>,
}

//~ struct ProjectTask {
    //~ id: Uuid,
    //~ name: String,
    //~ url: String,
    //~ ptchecks: VecDeque<PTCheck>,
//~ }

use yew::prelude::*;

pub struct Dashboard {
    link: ComponentLink<Self>,
    // ... other properties ...
}

pub enum Msg {
    // Define message types here
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            // Handle messages here
        }
    }

    //~ fn view(&self) -> Html {
        //~ html! {
            //~ // Dashboard layout and components go here
        //~ }
    //~ }

fn view(&self) -> Html {
    html! {
        <div class="dashboard">
            <div class="project-tasks">
                { for self.project_tasks.iter().map(|project_task| self.view_project_task(project_task)) }
            </div>
        </div>
    }
}

fn view_project_task(&self, project_task: &ProjectTask) -> Html {
    html! {
        <div class="project-task">
            <input type="checkbox" checked=project_task.selected />
            <span>{ &project_task.name }</span>
            <button onclick=self.link.callback(|_| Msg::ExpandProjectTask(project_task.id))>
                { "Expand" }
            </button>
            // Add more buttons for other actions here
            { if project_task.expanded {
                html! {
                    <div class="ptchecks">
                        { for project_task.ptchecks.iter().map(|ptcheck| self.view_ptcheck(ptcheck)) }
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn view_ptcheck(&self, ptcheck: &PTCheck) -> Html {
    html! {
        <div class="ptcheck">
            <input type="checkbox" checked=ptcheck.selected />
            <span>{ ptcheck.timestamp.format("%Y-%m-%d %H:%M:%S").to_string() }</span>
            // Add more buttons for other actions here
        </div>
    }
}


}
