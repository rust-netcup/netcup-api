use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    #[serde(rename = "serverrequestid")] 
    pub server_request_id: String,
    #[serde(rename = "clientrequestid")] 
    pub client_request_id: String,
    #[serde(rename = "action")] 
    pub action: String,
    #[serde(rename = "status")] 
    pub status: String,
    #[serde(rename = "statuscode")] 
    pub status_code: u32,
    #[serde(rename = "shortmessage")] 
    pub short_message: String,
    #[serde(rename = "longmessage")] 
    pub long_message: String,
    #[serde(rename = "responsedata")] 
    pub response_data: T,
}
