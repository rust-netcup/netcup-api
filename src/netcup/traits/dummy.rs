use async_trait::async_trait;

use crate::{Error, Request, Response, Client};

/// A dummy trait used to test the Client and as an example for how to implement actions. 
/// 
/// Using this trait **will result in errors**.  
/// **This is not a valid request and should not be used in production**.
#[async_trait]
pub trait Dummy {
    async fn dummy(
        some_value: &str,
    ) -> Result<Response<String>, Error>;
}

#[async_trait]
impl Dummy for Client {
    /// Send a dummy request to the API.  
    /// Using this **will result in errors**.  
    /// **This is not a valid request and should not be used in production**.
    async fn dummy(
        some_value: &str,
    ) -> Result<Response<String>, Error> {
        let dummy_request = String::from(some_value);
        let request = Request {
            action: String::from("dummy"),
            param: dummy_request,
        };
        let login_response = Self::send_request::<String, String>(request).await?;

        Ok(login_response)
    }
}
