use hyper::{
    Body as HyperBody, Client as HyperClient, Method as HyperMethod, Request as HyperRequest,
};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Request, Response};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Client {
    api_key: String,
    api_password: String,
    customer_number: u32,
    api_session_id: String,
}

impl Client {
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
        // -> I.e. most likely an issue at Netcup's CCP API. Server overload, API changes, etc.
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
