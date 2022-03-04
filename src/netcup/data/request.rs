use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    #[serde(rename = "action")] 
    pub action: String,
    #[serde(rename = "param")] 
    pub param: T,
}
