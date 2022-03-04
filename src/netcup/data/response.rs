use serde_derive::{Deserialize, Serialize};

/// A struct mirroring Netcup's API.
/// Use this as a base for parsing responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    /// A unique ID for the request.  
    /// Mostly unused.
    #[serde(rename = "serverrequestid")] 
    pub server_request_id: String,
    /// A unique ID for the request.  
    /// Mostly unused.
    #[serde(rename = "clientrequestid")] 
    pub client_request_id: String,
    /// The chosen action.  
    /// Mirrors the action field of the request.
    #[serde(rename = "action")] 
    pub action: String,
    /// A status, defining if the request was successful or not.  
    /// If you are looking for request validation, use the `statuscode` field instead.
    #[serde(rename = "status")] 
    pub status: String,
    /// A number stating the status code of the request.  
    /// 2000-2999 indicates a success.  
    /// 3000-3999 indicates a client-side error.  
    /// 4000-4999 indicates a server-side error.
    #[serde(rename = "statuscode")] 
    pub status_code: u32,
    /// A short message explaining the issue.
    #[serde(rename = "shortmessage")] 
    pub short_message: String,
    /// A long message explaining the issue.
    #[serde(rename = "longmessage")] 
    pub long_message: String,
    /// Additional response data.  
    /// Depending on your action request, various fields will be nested here.
    #[serde(rename = "responsedata")] 
    pub response_data: T,
}
