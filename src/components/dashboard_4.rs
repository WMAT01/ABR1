match user.database_type {
    DatabaseType::SurrealDB => {
        // Call SurrealDB functions from database.rs
    }
    DatabaseType::SQLite3 => {
        // Call SQLite3 functions from dbsqlite3.rs
    }
}
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::timeout::{TimeoutService, TimeoutTask};
use yew::format::{Json, Nothing};
use yew_router::{prelude::*, route::Route, switch::Permissive};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::components::keywords::Keywords;
use yew::events::InputData;
use yew::prelude::*;
use difference::{Changeset, Difference};
// Add any necessary imports

// src/dashboard.rs
use crate::database::{open_latest_database};

fn load_data(&mut self, email: &str) {
    let database_name = open_latest_database(email);
    // Load the data from the database and populate the UI
    // ...
}


pub struct Dashboard {
    link: ComponentLink<Self>,
    project_tasks: Vec<ProjectTask>,
    selected_project_tasks: Vec<String>,
    search_input: String,
    // other fields
}

pub enum Msg {
    AddProjectTask,
    DeleteProjectTask(String),
    RunReport(String),
    DownloadDatabase,
    CheckAllNow,
    ToggleTimerMode,
    ToggleProjectTaskSelection(String),
    SearchInputChanged(String),
    ClearSearchInput,
    // other messages
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    //~ fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        //~ // Initialize Dashboard
    //~ }
	//~ fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        //~ // Load project_tasks from the database
        //~ let project_tasks = vec![]; // Replace with actual data loading

        //~ Dashboard {
            //~ link,
            //~ project_tasks: Vec::new,
            //~ // Initialize other state variables
        //~ }
    //~ }
	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut dashboard = Dashboard {
            link,
            project_tasks: Vec::new(),
        };

        // Get the user's email (from authentication)
        let user_email = "example@example.com"; // TODO: Replace this with the actual authenticated user's email
        dashboard.load_data(user_email);

        dashboard
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddProjectTask => {
                // Add a new ProjectTask
            }
            Msg::DeleteProjectTask(id) => {
                // Delete the specified ProjectTask by ID
            }
            Msg::RunReport(id) => {
                // Run a report for the specified ProjectTask by ID
            }
            Msg::DownloadDatabase => {
                // Download the database
            }
            Msg::CheckAllNow => {
                // Check all ProjectTasks now
            }
            Msg::ToggleTimerMode => {
                // Toggle Timer Mode
            }
            Msg::ToggleProjectTaskSelection(id) => {
                // Toggle the selection of a ProjectTask by ID
            }
            Msg::SearchInputChanged(input) => {
                // Update the search input value
            }
            Msg::ClearSearchInput => {
                // Clear the search input value
            }
            // other messages
        }
    }
    
    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }
    
    fn view(&self) -> Html {
        html! {
            <>
                // Render Dashboard elements and functionality
                // Add, Delete, Run Report, Download Database, Check All Now, Timer Mode buttons or components
                <input type="text"
                       placeholder="Search keywords"
                       value=&self.search_input
                       oninput=self.link.callback(|e: InputData| Msg::SearchInputChanged(e.value)) />
                <button onclick=self.link.callback(|_| Msg::ClearSearchInput)>{"X"}</button>
                {for self.project_tasks.iter().map(|project_task| self.view_project_task(project_task))}
            </>
        }
    }
}

impl Dashboard {
    //~ fn view_project_task(&self, project_task: &ProjectTask) -> Html {
        //~ let is_selected = self.selected_project_tasks.contains(&project_task.id);

        //~ html! {
            //~ <div>
                //~ <input type="checkbox"
                       //~ checked=is_selected
                       //~ onclick=self.link.callback(move |_| Msg::ToggleProjectTaskSelection(project_task.id.clone())) />
                //~ <h3>{ &project_task.name }</h3>
                //~ <p>{ "URL: " }{ &project_task.url }</p>
                //~ <Keywords keywords=&project_task.keywords />
                //~ <button onclick=self.link.callback(move |_| Msg::RunReport(project_task.id.clone()))>{"Run Report"}</button>
                //~ {for project_task.ptchecks.iter().map(|ptcheck| self.view_ptcheck(ptcheck))}
            //~ </div>
        //~ }
    //~ }
    fn view_project_task(&self, project_task: &ProjectTask) -> Html {
        html! {
            <div>
                <h3>{ &project_task.name }</h3>
                <p>{ &project_task.url }</p>
                <p>{ "Keywords: " }{ project_task.keywords.join(", ") }</p>
                <button onclick=self.link.callback(move |_| Msg::Check(project_task.id.clone()))>{"Check"}</button>
                <button onclick=self.link.callback(move |_| Msg::RunReport(project_task.id.clone()))>{"Run Report"}</button>
                {for project_task.ptchecks.iter().map(|ptcheck| self.view_ptcheck(ptcheck))}
            </div>
        }
    }

    //~ fn view_ptcheck(&self, ptcheck: &PTCheck) -> Html {
        //~ html! {
            //~ <div>
                //~ <h4>{ &ptcheck.timestamp }</h4>
                //~ <p>{ "Status: " }{ &ptcheck.status }</p>
            //~ </div>
        //~ }
    //~ }
//~ }
    fn view_ptcheck(&self, ptcheck: &PTCheck) -> Html {
        let diff = self.compute_diff(ptcheck);
        html! {
            <div>
                <h4>{ &ptcheck.timestamp }</h4>
                <p>{ "Status: " }{ &ptcheck.status }</p>
                <pre>{ diff }</pre>
            </div>
        }
    }

	fn compute_diff(&self, ptcheck: &PTCheck) -> String {
		// Decompress the compressed_response (you can use the `lzma` crate)
		// For demonstration purposes, I'm using some dummy text here
		let old_text = "This is the old text";
		let new_text = "This is the new text";

		let changeset = Changeset::new(old_text, new_text, "\n");

		let mut diff_output = String::new();
		for diff in changeset.diffs {
			match diff {
				Difference::Same(ref x) => {
					diff_output.push_str(x);
					diff_output.push('\n');
				}
				Difference::Add(ref x) => {
					diff_output.push_str("+");
					diff_output.push_str(x);
					diff_output.push('\n');
				}
				Difference::Rem(ref x) => {
					diff_output.push_str("-");
					diff_output.push_str(x);
					diff_output.push('\n');
				}
			}
		}

		diff_output
	}
	
	
}


// Add the necessary structs and other code

#[derive(Clone, PartialEq, Properties)]
pub struct ProjectTask {
    pub id: String,
    pub name: String,
    pub url: String,
    pub keywords: Vec<String>,
    pub ptchecks: Vec<PTCheck>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PTCheck {
    pub timestamp: String,
    pub status: String,
    pub compressed_response: Vec<u8>,
}
