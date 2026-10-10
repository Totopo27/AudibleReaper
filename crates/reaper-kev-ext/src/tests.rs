use crate::ipc::{CommandPayload, CommandResponse};

#[test]
fn test_command_payload_serialization() {
    let payload = CommandPayload {
        action_id: 40044,
        track_index: Some(1),
        description: Some("Play/Stop Transport".to_string()),
    };

    let json = serde_json::to_string(&payload).expect("Failed to serialize payload");
    assert!(json.contains("40044"));
    assert!(json.contains("Play/Stop Transport"));

    let deserialized: CommandPayload =
        serde_json::from_str(&json).expect("Failed to deserialize payload");
    assert_eq!(deserialized.action_id, 40044);
    assert_eq!(deserialized.track_index, Some(1));
    assert_eq!(
        deserialized.description.as_deref(),
        Some("Play/Stop Transport")
    );
}

#[test]
fn test_command_response_serialization() {
    let resp = CommandResponse {
        success: true,
        message: "Action queued".to_string(),
    };

    let json = serde_json::to_string(&resp).expect("Failed to serialize response");
    let deserialized: CommandResponse =
        serde_json::from_str(&json).expect("Failed to deserialize response");
    assert!(deserialized.success);
    assert_eq!(deserialized.message, "Action queued");
}
