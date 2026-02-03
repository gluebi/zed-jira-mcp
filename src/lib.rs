use std::env;
use zed_extension_api::{
    self as zed, Command, ContextServerConfiguration, ContextServerId, Project, Result,

};

const PACKAGE_NAME: &str = "mcp-remote";
const PACKAGE_VERSION: &str = "0.1.37";
const PACKAGE_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
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
        // Install mcp-remote package if not already installed or wrong version
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        // Get the path to the proxy.js file
        let proxy_path = env::current_dir()
            .unwrap()
            .join(PACKAGE_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![proxy_path, JIRA_MCP_URL.to_string()],
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
            installation_instructions: r#"# Jira MCP Server

This extension connects Zed to a locally running Jira MCP Server for issue management.

## Requirements

- Node.js v22 or later
- Jira MCP Server running locally on port 3010
- Jira Cloud instance with OAuth 2.0 app configured

## Setup

1. Start the Jira MCP Server:
   cd ~/development/mp/jira-mcp-server
   docker-compose up -d
   
   Or run locally:
   pnpm run dev

2. Authenticate with Jira:
   - Open http://localhost:3010 in your browser
   - Click "Connect to Jira"
   - Log in to Atlassian and authorize the app

3. Enable the context server in Zed Agent Panel settings

## Available Tools

### Authentication
- jira_authenticate: Start OAuth flow
- jira_auth_status: Check authentication status
- jira_logout: Disconnect from Jira

### Issue Management
- jira_get_issue: Get issue by key (e.g., PROJ-123)
- jira_create_issue: Create a new issue
- jira_update_issue: Update issue fields
- jira_delete_issue: Delete an issue
- jira_search_issues: Search using JQL
- jira_get_transitions: Get available status transitions
- jira_transition_issue: Change issue status

### Comments
- jira_get_comments: Get comments on an issue
- jira_add_comment: Add a comment
- jira_update_comment: Edit a comment
- jira_delete_comment: Delete a comment

### Extended Features
- jira_get_attachments: List issue attachments
- jira_get_worklogs: Get time logs
- jira_add_worklog: Log time on issue
- jira_get_issue_links: Get linked issues
- jira_create_issue_link: Link two issues

## Troubleshooting

If connection fails:
1. Ensure Jira MCP Server is running on port 3010
2. Check if authenticated at http://localhost:3010
3. Check the server logs: docker-compose logs -f
4. Restart Zed and re-enable the context server
"#.to_string(),
            default_settings: default_settings.to_string(),
            settings_schema: settings_schema.to_string(),
        }))
    }
}

zed::register_extension!(JiraMcpServer);
