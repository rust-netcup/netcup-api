use async_trait::async_trait;

use crate::{Error, Request, Response, Client};

#[async_trait]
pub trait Dummy {
    async fn dummy(
        some_value: &str,
    ) -> Result<Response<String>, Error>;
}

#[async_trait]
impl Dummy for Client {
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
