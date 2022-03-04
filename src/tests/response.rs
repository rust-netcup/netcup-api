#[test]
fn serialize_with_values() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"serverrequestid\":\"Test ServerRequestID\""));
    assert!(response_json.contains("\"clientrequestid\":\"Test ClientRequestID\""));
    assert!(response_json.contains("\"action\":\"Test Action\""));
    assert!(response_json.contains("\"status\":\"Test Status\""));
    assert!(response_json.contains("\"statuscode\":1234567890"));
    assert!(response_json.contains("\"shortmessage\":\"Test ShortMessage\""));
    assert!(response_json.contains("\"longmessage\":\"Test LongMessage\""));
    assert!(response_json.contains("\"responsedata\":\"Test Data\""));
}

#[test]
fn validate_field_server_request_id() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"serverrequestid\":\""));
}


#[test]
fn validate_field_client_request_id() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"clientrequestid\":\""));
}

#[test]
fn validate_field_action() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"action\":\""));
}

#[test]
fn validate_field_status() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"status\":\""));
}

#[test]
fn validate_field_status_code() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"statuscode\":"));
}

#[test]
fn validate_field_short_message() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"shortmessage\":\""));
}

#[test]
fn validate_field_long_message() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"longmessage\":\""));
}

#[test]
fn validate_field_response_data() {
    use crate::Response;

    let response = Response::<String> {
        server_request_id: "Test ServerRequestID".to_string(),
        client_request_id: "Test ClientRequestID".to_string(),
        action: "Test Action".to_string(),
        status: "Test Status".to_string(),
        status_code: 1234567890,
        short_message: "Test ShortMessage".to_string(),
        long_message: "Test LongMessage".to_string(),
        response_data: "Test Data".to_string(),
    };

    let response_json = serde_json::to_string(&response).unwrap();
    dbg!("{:#?}", &response_json);

    assert!(response_json.contains("\"responsedata\":\""));
}
