#[cfg(feature = "schemars")]
mod tests {
    use rmcp::model::{ClientJsonRpcMessage, ServerJsonRpcMessage};
    use super::*;
    use schemars::schema_for;

    #[test]
    fn test_client_json_rpc_message_schema() {
        let schema = schema_for!(ClientJsonRpcMessage);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        let expected = std::fs::read_to_string("tests/test_message_schema/client_json_rpc_message_schema.json").unwrap();
        assert_eq!(schema_str, expected, "Schema generation for ClientJsonRpcMessage should match expected output");
    }

    #[test]
    fn test_server_json_rpc_message_schema() {
        let schema = schema_for!(ServerJsonRpcMessage);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        let expected = std::fs::read_to_string("tests/test_message_schema/server_json_rpc_message_schema.json").unwrap();
        assert_eq!(schema_str, expected, "Schema generation for ServerJsonRpcMessage should match expected output");
    }
}
