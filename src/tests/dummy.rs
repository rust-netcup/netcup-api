#[test]
fn dummy() {
    use crate::{Client, Dummy};
    let rt = tokio::runtime::Runtime::new().unwrap();

    let future = Client::dummy("some_value");
    let result = rt.block_on(future);

    // This should fail as it's a dummy and invalid request. 
    // It's intended to validate that the client is working correctly
    // and that the used approach is indeed working for this library.
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Server-Side Error"));
}
