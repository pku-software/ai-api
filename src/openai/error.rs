use serde_json::json;

pub(super) fn parse_json_error() -> serde_json::Value {
    json!(
        {
            "status": "failed",
            "text": "Error when parse JSON"
        }
    )
}

pub(super) fn network_error() -> serde_json::Value {
    json!(
        {
            "status": "failed",
            "text": "Network error"
        }
    )
}
