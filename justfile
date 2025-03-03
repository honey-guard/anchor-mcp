ping:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "ping" }' | ./target/debug/anchor-mcp --mcp

prompts-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/list" }' | ./target/debug/anchor-mcp --mcp

prompt-get:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/get", "params": {"name":"security_check_program","arguments": {"program_path": "./ignore_examples/sealevel-attacks/programs/3-type-cosplay/insecure"} } }' | ./target/debug/anchor-mcp --mcp

tools-list:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/list" }' | ./target/debug/anchor-mcp --mcp

security-check-program:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "security_check_program", "arguments": {"program_path": "./ignore_examples/sealevel-attacks/programs/3-type-cosplay/insecure"} } }' | ./target/debug/anchor-mcp --mcp

security-check-file:
  echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "security_check_file", "arguments": {"file_path":"./ignore_examples/sealevel-attacks/programs/3-type-cosplay/insecure/src/lib.rs"} } }' | ./target/debug/anchor-mcp --mcp
