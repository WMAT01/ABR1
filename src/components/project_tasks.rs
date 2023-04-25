use yew::prelude::*;
use yew::services::ConsoleService;

pub struct ProjectTask {
    link: ComponentLink<Self>,
    props: Props,
    is_edit_modal_open: bool,
    edited_name: String,
    edited_url: String,
    edited_keywords: String,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub keywords: Vec<String>,
    pub on_remove: Callback<usize>,
}

pub enum Msg {
    Remove,
    OpenEditModal,
    CloseEditModal,
    UpdateEditedName(String),
    UpdateEditedUrl(String),
    UpdateEditedKeywords(String),
    SaveTask,
}

impl Component for ProjectTask {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            is_edit_modal_open: false,
            edited_name: props.name.clone(),
            edited_url: props.url.clone(),
            edited_keywords: props.keywords.join(", "),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Remove => {
                self.props.on_remove.emit(self.props.id);
            }
            Msg::OpenEditModal => {
                self.is_edit_modal_open = true;
            }
            Msg::CloseEditModal => {
                self.is_edit_modal_open = false;
            }
            Msg::UpdateEditedName(new_name) => {
                self.edited_name = new_name;
            }
            Msg::UpdateEditedUrl(new_url) => {
                self.edited_url = new_url;
            }
            Msg::UpdateEditedKeywords(new_keywords) => {
                self.edited_keywords = new_keywords;
            }
            Msg::SaveTask => {
                // Save the updated task information
                // TODO: Save the edited task information to the database

                self.props.name = self.edited_name.clone();
                self.props.url = self.edited_url.clone();
                self.props.keywords = self
                    .edited_keywords
                    .split(',')
                    .map(|kw| kw.trim().to_string())
                    .collect();
                self.is_edit_modal_open = false;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="project-task">
                    <h2>{ &self.props.name }</h2>
                    <p>{ &self.props.url }</p>
                    <p>{ self.props.keywords.iter().map(|kw| format!(", {}", kw)).collect::<String>() }</p>
                    <button onclick=self.link.callback(|_| Msg::OpenEditModal)>{"Edit"}</button>
                    <button onclick=self.link.callback(|_| Msg::Remove)>{"Remove"}</button>
                </div>
                { self.view_edit_modal() }
            </>
        }
    }
}

impl ProjectTask {
    fn view_edit_modal(&self) -> Html {
        if self.is_edit_modal_open {
            html! {
                <div class="edit-modal">
                    <div class="edit-modal-content">
                        <h2>{"Edit Task"}</h2>
                        <label>{"Name"}</label>
                        <input
                            type="text"
                            value=&self.edited_name
                            oninput=self.link.callback(|e: InputData| Msg::UpdateEditedName(e.value))
                        />
                        <label>{"URL"}</label>
							<input
						<input
							type="text"
							value=&self.edited_url
							oninput=self.link.callback(|e: InputData| Msg::UpdateEditedUrl(e.value))
						/>
						<label>{"Keywords"}</label>
						<input
							type="text"
							value=&self.edited_keywords
							oninput=self.link.callback(|e: InputData| Msg::UpdateEditedKeywords(e.value))
						/>
						<button onclick=self.link.callback(|_| Msg::SaveTask)>{"Save"}</button>
						<button onclick=self.link.callback(|_| Msg::CloseEditModal)>{"Cancel"}</button>
					</div>
				</div>
			}
    } else {
        html! {}
    }
}
