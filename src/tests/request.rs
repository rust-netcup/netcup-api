#[test]
fn serialize_with_values() {
    use crate::Request;

    let request = Request::<String> {
        action: "Test Action".to_string(),
        param: "Test Param".to_string(),
    };

    let request_json = serde_json::to_string(&request).unwrap();
    dbg!("{:#?}", &request_json);

    assert!(request_json.contains("\"action\":\"Test Action\""));
    assert!(request_json.contains("\"param\":\"Test Param\""));
}

#[test]
fn validate_field_action() {
    use crate::Request;

    let request = Request::<String> {
        action: "Test Action".to_string(),
        param: "Test Param".to_string(),
    };

    let request_json = serde_json::to_string(&request).unwrap();
    dbg!("{:#?}", &request_json);

    assert!(request_json.contains("\"action\":\""));
}


#[test]
fn validate_field_param() {
    use crate::Request;

    let request = Request::<String> {
        action: "Test Action".to_string(),
        param: "Test Param".to_string(),
    };

    let request_json = serde_json::to_string(&request).unwrap();
    dbg!("{:#?}", &request_json);

    assert!(request_json.contains("\"param\":\""));
}
