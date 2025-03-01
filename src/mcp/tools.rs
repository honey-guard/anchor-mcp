use crate::mcp::types::*;
use maplit::hashmap;
use rpc_router::{Handler, HandlerResult, RouterBuilder};

use solana_fender::{analyze_program_file, analyze_program_dir, findings_to_markdown};
use std::path::PathBuf;
/// register all tools to the router
pub fn register_tools(router_builder: RouterBuilder) -> RouterBuilder {
    router_builder
        .append_dyn("security_check_program", security_check_program.into_dyn())
        .append_dyn("security_check_file", security_check_file.into_dyn())
        .append_dyn("tools/list", tools_list.into_dyn())
}

pub async fn tools_list(_request: Option<ListToolsRequest>) -> HandlerResult<ListToolsResult> {
    //let tools: Vec<Tool> = serde_json::from_str(include_str!("./templates/tools.json")).unwrap();
    let response = ListToolsResult {
        tools: vec![
        Tool {
            name: "security_check_program".to_string(),
            description: Some("Check the anchor program for security issues".to_string()),
            input_schema: ToolInputSchema {
                type_name: "object".to_string(),
                properties: hashmap! {
                    "program_path".to_string() => ToolInputSchemaProperty {
                        type_name: Some("string".to_owned()),
                        description: Some("program path".to_owned()),
                        enum_values: None,
                    }
                },
                required: vec!["program_path".to_string()],
            },
        },
        Tool {
            name: "security_check_file".to_string(),
            description: Some("Check the anchor file for security issues".to_string()),
            input_schema: ToolInputSchema {
                type_name: "object".to_string(),
                properties: hashmap! {
                    "file_path".to_string() => ToolInputSchemaProperty {
                        type_name: Some("string".to_owned()),
                        description: Some("file path".to_owned()),
                        enum_values: None,
                    }
                },  
                required: vec!["file_path".to_string()],
            },
        },  
        
        ],
        next_cursor: None,
    };
    Ok(response)
}

pub async fn security_check_program(params: SecurityCheckProgramRequest) -> HandlerResult<CallToolResult> {
    let program_path = params.program_path.unwrap();
    let findings = analyze_program_dir(PathBuf::from(&program_path)).unwrap();
    let markdown = findings_to_markdown(findings, "Program", None).unwrap();

    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: markdown }],
        is_error: false,
    })
}

pub async fn security_check_file(params: SecurityCheckFileRequest) -> HandlerResult<CallToolResult> {
    let file_path = params.file_path.unwrap();
    let findings = analyze_program_file(PathBuf::from(&file_path)).unwrap();
    let markdown = findings_to_markdown(findings, "Program", None).unwrap();
    Ok(CallToolResult {
        content: vec![CallToolResultContent::Text { text: markdown }],
        is_error: false,
    })
}


