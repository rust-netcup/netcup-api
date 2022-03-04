use hyper::{
    Body as HyperBody, Client as HyperClient, Method as HyperMethod, Request as HyperRequest,
};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Request, Response};

/// This client is used to send requests to the Netcup API and parse the responses.
/// The client is written as dynamic as possible and only features a few functions.
/// Said functions are used for communication with the Netcup API.
/// 
/// Furthermore, a few basic variable fields are used to store important information such as login data (api key, api secret/password, customer number) and once a login succeeded its session id.
/// These variables will be used by subsequent requests to authenticate.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Client {
    /// Netcup API Key -- Read documentation on lib.rs/main-page for more information
    api_key: String,
    /// Netcup API Secret/Password -- Read documentation on lib.rs/main-page for more information
    api_password: String,
    /// Netcup CCP Customer Number
    customer_number: u32,
    /// Netcup CCP Session ID
    api_session_id: String,
}

impl Client {
    /// Used to send a request to Netcup's API using JSON.
    /// This function is dynamically written, make sure to use the correct Request- and Response-Types.
    /// Parsing errors are most likely caused by the wrong Request- or Response-Type.
    /// 
    /// Netcup's API uses a single endpoint for all JSON requests:
    /// <https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON>
    /// 
    /// It is expected to send a valid JSON-formatted request to said endpoint.
    /// The class `Request` is used to mirror this structure and is exclusively used by this function.
    /// Similarly, the class `Response` is used to mirror any responses from the API.
    /// 
    /// Make sure to check the documentation on both structs for more information.
    /// 
    /// # Usage
    /// 
    /// This function rarely if ever should be called directly.  
    /// Instead, traits are used to implement individual 'endpoints' (API actions).
    /// 
    /// ```rust
    /// use netcup_api::*;
    /// 
    /// // Create a request (in this case a simple string):
    /// let dummy_request = String::from("some value");
    /// 
    /// // Construct a request object:
    /// let request = Request::<String> { // The <String> is our Request-Type
    ///     action: String::from("dummy"),
    ///     param: dummy_request,
    /// };
    /// 
    /// // We are using a String as our Request-Type and our Response-Type.
    /// // Send the request and receive a response:
    /// let response = Client::send_request::<String, String>(request);
    /// // `response` is a Future.  
    /// // Use `.await` to get the actual response in an async context or
    /// // create a tokio thread: 
    /// let rt = tokio::runtime::Runtime::new().unwrap();
    /// let actual_response = rt.block_on(response);
    /// // Now it's a Result!
    /// ```
    /// 
    /// > NOTE: **THIS REQUEST WILL FAIL. IT IS NOT VALID!**  
    /// > It is only an example on _how_ to use this function.
    pub async fn send_request<RequestType: Serialize, ResponseType: DeserializeOwned>(
        request: Request<RequestType>,
    ) -> Result<Response<ResponseType>, Error> {
        // --- Hyper.rs client setup ---
        let https_connector = HttpsConnector::new();
        let hyper_client = HyperClient::builder().build::<_, HyperBody>(https_connector);

        // --- Request ---
        let request_json = serde_json::to_string(&request).unwrap();
        dbg!("Request:\n{:#?}", &request_json);

        let request = HyperRequest::builder()
            .method(HyperMethod::POST)
            .uri("https://ccp.netcup.net/run/webservice/servers/endpoint.php?JSON")
            .header("content-type", "application/json")
            .body(HyperBody::from(request_json))
            .map_err(|e| e.to_string())?;

        // --- Response ---
        let response = hyper_client
            .request(request)
            .await
            .map_err(|e| e.to_string())?;
        // Read to bytes
        let response_bytes = hyper::body::to_bytes(response.into_body())
            .await
            .map_err(|e| e.to_string())?;
        // Read to String (endpoint uses JSON, thus JSON!)
        let response_json = String::from_utf8(response_bytes.to_vec())
            .expect("failed while parsing response data to string");
        dbg!("Response:\n{:#?}", &response_json);

        // Convert JSON to a base response object.
        // We tell serde here that everything inside the additional data field is "just a string", instead of a nested object.
        // This makes it easy for us to read the various fields of the response object. Like it's status-code and messages.
        let response_base_object: Response<String> =
            serde_json::from_str::<Response<String>>(&response_json).map_err(|e| e.to_string())?;

        // Process status-code.
        // Everything between 2000 and 3000 is a success message.
        // Everything between 3000 and 4000 is a client-side error message.
        // -> I.e. either this library does something wrong or (after validations and such) the provided login data is invalid.
        // Everything between 4000 and 5000 is a server-side error message.
        // -> I.e. most likely an issue at Netcup's API. Server overload, API changes, etc.
        if response_base_object.status_code >= 2000 && response_base_object.status_code < 3000 {
            // Success
            Ok(serde_json::from_str::<Response<ResponseType>>(&response_json).map_err(|e| e.to_string())?)
        } else if response_base_object.status_code >= 3000 && response_base_object.status_code < 4000 {
            // Client-side error
            Err(Error::from(format!(
                "Client-Side Error! Are API credentials correct & not rate limited? Response: {}",
                response_json
            )))
        } else if response_base_object.status_code >= 4000 && response_base_object.status_code < 5000 {
            // Server-side error
            Err(Error::from(format!(
                "Server-Side Error! Is Netcup alright? Response: {}",
                response_json
            )))
        } else {
            // Unknown error
            Err(Error::from(format!(
                "Unknown Error! Is... Netcup alright??? Response: {}",
                response_json
            )))
        }
    }
}
