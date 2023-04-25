use yew::prelude::*;

pub struct Profile {
    email: String,
    provider: String,
    database_size: String,
    project_task_count: u32,
}

pub enum Msg {}

impl Component for Profile {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        // TODO: Get the user's email, provider, database size, and project task count
        Self {
            email: "example@example.com".to_string(),
            provider: "Google".to_string(),
            database_size: "2.5 MB".to_string(),
            project_task_count: 42,
        }
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
                <h1>{ "Profile" }</h1>
                <p>{ format!("Email: {}", self.email) }</p>
                <p>{ format!("Provider: {}", self.provider) }</p>
                <p>{ format!("Database size: {}", self.database_size) }</p>
                <p>{ format!("Project tasks: {}", self.project_task_count) }</p>
                <a href="/billing">{ "Go to Billing" }</a>
                <button onclick=self.link.callback(|_| Msg::DownloadDatabase)>{ "Download Database" }</button>
            </>
        }
    }
}

		
