use std::fs;
use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const JIRA_MCP_URL: &str = "http://localhost:3010/mcp";

struct JiraMcpServer;

impl zed::Extension for JiraMcpServer {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Command> {
        let extension_dir = env::current_dir().unwrap();
        let bridge_path = extension_dir.join("stdio-bridge.js");
        
        if !bridge_path.exists() {
            let bridge_script = include_str!("../stdio-bridge.js");
            fs::write(&bridge_path, bridge_script)
                .map_err(|e| format!("Failed to write bridge script: {}", e))?;
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                bridge_path.to_string_lossy().to_string(),
                JIRA_MCP_URL.to_string(),
            ],
            env: vec![],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let settings_schema = serde_json::json!({});
        let default_settings = serde_json::json!({});

        Ok(Some(ContextServerConfiguration {
            installation_instructions: "Jira MCP - requires server on port 3010".to_string(),
            default_settings: default_settings.to_string(),
            settings_schema: settings_schema.to_string(),
        }))
    }
}

zed::register_extension!(JiraMcpServer);
