use serde_derive::{Deserialize, Serialize};

/// A struct mirroring Netcup's API.
/// Use this as a base for creating requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    /// The action to be performed.
    /// Check [Netcup's endpoint documentation](https://ccp.netcup.net/run/webservice/servers/endpoint.php)for all available actions.
    #[serde(rename = "action")] 
    pub action: String,
    /// Additional parameters, usually a nested (later: JSON) object.
    /// Depending on the chosen actions certain fields are required inside the `param` field.
    #[serde(rename = "param")] 
    pub param: T,
}
