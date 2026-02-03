#!/usr/bin/env node
/**
 * Simple stdio-to-HTTP bridge for MCP servers
 * Reads JSON-RPC messages from stdin, sends to HTTP endpoint, writes response to stdout
 */

const http = require('http');
const readline = require('readline');

const MCP_URL = process.argv[2] || 'http://localhost:3010/mcp';
const url = new URL(MCP_URL);

let sessionId = null;

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

function sendRequest(message) {
  return new Promise((resolve, reject) => {
    const data = JSON.stringify(message);
    
    const headers = {
      'Content-Type': 'application/json',
      'Accept': 'application/json, text/event-stream',
    };
    
    if (sessionId) {
      headers['Mcp-Session-Id'] = sessionId;
    }
    
    const options = {
      hostname: url.hostname,
      port: url.port || 80,
      path: url.pathname,
      method: 'POST',
      headers: headers,
    };

    const req = http.request(options, (res) => {
      // Capture session ID from response
      const newSessionId = res.headers['mcp-session-id'];
      if (newSessionId) {
        sessionId = newSessionId;
      }
      
      let body = '';
      res.on('data', chunk => body += chunk);
      res.on('end', () => {
        // Handle SSE format
        if (body.startsWith('event:')) {
          const lines = body.split('\n');
          for (const line of lines) {
            if (line.startsWith('data: ')) {
              const jsonData = line.slice(6);
              if (jsonData) {
                resolve(JSON.parse(jsonData));
                return;
              }
            }
          }
        }
        // Handle plain JSON
        try {
          resolve(JSON.parse(body));
        } catch (e) {
          reject(new Error(`Invalid response: ${body}`));
        }
      });
    });

    req.on('error', reject);
    req.write(data);
    req.end();
  });
}

rl.on('line', async (line) => {
  if (!line.trim()) return;
  
  try {
    const message = JSON.parse(line);
    const response = await sendRequest(message);
    console.log(JSON.stringify(response));
  } catch (error) {
    const errorResponse = {
      jsonrpc: '2.0',
      error: {
        code: -32603,
        message: error.message
      },
      id: null
    };
    console.log(JSON.stringify(errorResponse));
  }
});

rl.on('close', () => {
  process.exit(0);
});
