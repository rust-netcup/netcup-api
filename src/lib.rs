// --- Docs ---
//! A simple library for working with the Netcup API.
//! 
//! # Credentials
//! To use this library you need to have a valid Netcup account and three things:
//! - A valid API key
//! - A valid API secret/password
//! - Your Netcup customer number
//! 
//! To generate the API key and API secret/password you can follow [Netcup's guide](https://www.netcup-wiki.de/wiki/CCP_API#Beantragung_eines_API-Passworts_und_API-Keys).  
//! However, it's German, so you might need to translate the guide to English.
//! 
//! Simplified this is what you need to do:
//! 1. Login into your account at [customercontrolpanel.de](https://customercontrolpanel.de/)
//! 2. Go to "Stammdaten" (EN: contact-data?)
//! 3. In the top right corner click on "API" ([direct link](https://www.customercontrolpanel.de/daten_aendern.php?sprung=api))
//! 4. Click on "API-Password generieren" (EN: Generate API-Password)  
//!     Make sure to write the Password/Secret down
//! 5. Accept the license and click on "API-Key erstellen" (EN: Generate API-Key)  
//!     Again: Make sure to write it down!
//! 6. Lastly, your customer number is at the top next to your name in brackets.  
//!     However, you also need that number to login into your account so you should have it already...
//! 
//! 
//! Once all the information is gathered, write it to a config file (`config.toml`) and have it in the apps folder:
//! ```toml
//! api_key = "your_api_key"
//! api_password = "your_api_password"
//! customer_number = your_customer_number
//! ``` 
//! 
//! ## Using this library
//! 
//! For ease of use, please make sure that if you package this library, you include a description on how to create and get the above information, link to this article or copy-and-paste it into your documentation.
//! 
//! # Netcup's API
//! 
//! The Netcup's API is a RESTful API that has several endpoints.  
//! The primary function is to handle domains and DNS entries.  
//! 
//! This library only uses JSON, but other protocols are also supported ([see here](https://www.netcup-wiki.de/wiki/CCP_API#API-Endpoint)).  
//! 
//! For JSON only one endpoint is used.  
//! Each action you want to perform uses a different children structure (request & response).
//! Overall, the API uses the following structure for requests:
//! ```json
//! // Request
//! {
//!     "action": "YOUR_ACTION",                // the action to be performed
//!     "param": ...,
//! }
//! ```
//! Where `param` can be anything: A String, a number or a nested (child) JSON.  
//! E.g. this is also possible (and often used!):
//! ```json
//! // Request
//! {
//!     "action": "YOUR_ACTION",                // the action to be performed
//!     "param": {
//!        "param1": "value1",
//!        "param2": "value2",
//!        "param3": "value3"
//!     },
//! }
//! ```
//! 
//! For responses the structure is the following:
//! ```json
//! {
//!    "serverrequestid": "server_request_id",  // can be ignored for now
//!    "clientrequestid": "client_request_id",  // can be ignored for now
//!    "action": "YOUR_ACTION",                 // the action to be performed
//!    "status": "some_status",                 // can be ignored for now
//!    "statuscode": 2000,                      // status code, see below
//!    "shortmessage": "some_message",          // A short message explaining the issue
//!    "longmessage": "some_message",           // A longer message explaining the issue
//!    "responsedata": ...,                     
//! }
//! ```
//! Once again, `responsedata` can be anything. 
//! Most of the time it'll be a nested JSON Object.
//! 
//! The whole documentation on all the actions can be found [here](https://ccp.netcup.net/run/webservice/servers/endpoint.php).
//! 
//! ## Status Codes
//! 
//! Netcup's API uses three categories of status codes:
//! 
//! - 2000-2999: Success
//! - 3000-3999: Client-Side Error (e.g. invalid credentials)
//! - 4000-4999: Server-Side Error (e.g. rate limit exceeded)
//! 

// --- Tests ---
#[cfg(test)]
mod tests;

// --- NetCup API ---
mod netcup;
pub use netcup::*;

// --- Lib Root definitions ---
type Error = String;

#[allow(dead_code)]
type NetcupClient = Client;
#[allow(dead_code)]
type CCPClient = Client;
