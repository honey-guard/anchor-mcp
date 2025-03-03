MCP Rust CLI server template
=============================

This is a MCP CLI server template for Anchor programs.

Model Context Protocol (MCP) is an open protocol that enables seamless integration between LLM applications
and external data sources and tools. Whether you’re building an AI-powered IDE, enhancing a chat interface,
or creating custom AI workflows, MCP provides a standardized way to connect LLMs with the context they need.

This server uses [mcp-rs-template.](https://github.com/linux-china/mcp-rs-template)

# CLI options

* `--mcp`: Enable MCP server
* `--resources`: display resources
* `--prompts`: display prompts
* `--tools`: display tools

# How to use Anchor MCP CLI server in Claude Desktop?

1. Edit `claude_desktop_config.json`: Claude Desktop -> `Settings` -> `Developer` -> `Edit Config` 
2. Add the following configuration to the `servers` section:

```json
{
   "mcpServers": {
      "security_check_program": {
         "command": "security_check_program",
         "args": [
            "--mcp"
         ]
      },
      "security_check_file": {
         "command": "security_check_file",
         "args": [
            "--mcp"
         ]
      }
   }
}
```

If you want to check MCP log, please use `tail -n 20 -f ~/Library/Logs/Claude/mcp*.log`.


# References

** Solana Fender: https://github.com/honey-guard/solana-fender
** mcp-rs-tempalte https://github.com/linux-china/mcp-rs-template
