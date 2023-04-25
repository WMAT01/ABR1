match user.database_type {
    DatabaseType::SurrealDB => {
        // Call SurrealDB functions from database.rs
    }
    DatabaseType::SQLite3 => {
        // Call SQLite3 functions from dbsqlite3.rs
    }
}
use crate::components::modal::Modal;
use yew::prelude::*;

pub struct ProjectTask {
    link: ComponentLink<Self>,
    props: Props,
    is_edit_modal_open: bool,
    edited_name: String,
    edited_url: String,
    edited_keywords: String,
}

// ... Props and Msg definitions

impl Component for ProjectTask {
    // ... create, update, change functions

    fn view(&self) -> Html {
        let on_modal_close = self.link.callback(|_| Msg::CloseEditModal);

        html! {
            <>
                // ... existing content

                <Modal is_open=self.is_edit_modal_open on_close=on_modal_close>
                    <h3>{ "Edit Task" }</h3>
                    <input type="text" value=&self.edited_name
                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditedName(e.value)) />
                    <input type="text" value=&self.edited_url
                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditedUrl(e.value)) />
                    <input type="text" value=&self.edited_keywords
                        oninput=self.link.callback(|e: InputData| Msg::UpdateEditedKeywords(e.value)) />
                    <button onclick=self.link.callback(|_| Msg::SaveTask)>{ "Save" }</button>
                </Modal>
            </>
        }
    }
}

