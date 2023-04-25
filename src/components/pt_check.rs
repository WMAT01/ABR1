use serde::{Deserialize, Serialize};
use crate::types::{PTCheckStatus, VisualDiffStatus, TextDiffStatus, WebDiffStatus};
use uuid::Uuid;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PTCheck {
    pub id: Uuid,
    pub project_task_id: Uuid,
    pub timestamp: SystemTime,
    pub compressed_response: Vec<u8>, // LZMA compressed response
    pub text_diff_status: TextDiffStatus,
    pub visual_diff_status: VisualDiffStatus,
    pub web_diff_status: WebDiffStatus,
}

impl PTCheck {
    pub fn new(
        project_task_id: Uuid,
        compressed_response: Vec<u8>,
        text_diff_status: TextDiffStatus,
        visual_diff_status: VisualDiffStatus,
        web_diff_status: WebDiffStatus,
    ) -> Self {
        PTCheck {
            id: Uuid::new_v4(),
            project_task_id,
            timestamp: SystemTime::now(),
            compressed_response,
            text_diff_status,
            visual_diff_status,
            web_diff_status,
        }
    }
}
