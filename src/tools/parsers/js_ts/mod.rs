use ollama_rs::function;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::{error::Error, path::PathBuf};
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::{Decl, ModuleItem, Stmt};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax, TsSyntax, lexer::Lexer};

/// Parses a JavaScript/TypeScript file and extracts function definitions.
pub fn parse_js_file(file_path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    let source_map = SourceMap::default();

    let fm = source_map.new_source_file(Rc::new(FileName::Real(file_path.to_path_buf())), content);

    let syntax = if file_path.extension().and_then(|ext| ext.to_str()) == Some("ts") {
        Syntax::Typescript(TsSyntax::default()) // Parse as TypeScript
    } else {
        Syntax::Es(EsSyntax::default()) // Parse as JavaScript
    };

    let lexer = Lexer::new(syntax, Default::default(), StringInput::from(&*fm), None);
    let mut parser = Parser::new_from(lexer);

    let module = parser
        .parse_module()
        .map_err(|e| format!("Parse error: {:?}", e))?;

    let mut functions = Vec::new();

    for stmt in module.body {
        if let ModuleItem::Stmt(Stmt::Decl(Decl::Fn(f))) = stmt {
            let func_name = f.ident.sym.to_string();
            functions.push(format!(
                "Function: {}({:?})",
                func_name,
                f.function
                    .params
                    .iter()
                    .map(|p| format!("{:?}", p))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
    }

    Ok(functions)
}

/// High-level function that summarizes JavaScript or TypeScript code based on the file type .
#[function]
pub async fn summarize_js_or_ts_code(
    file_path: String,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let path = PathBuf::from(file_path);

    match parse_js_file(&path) {
        Ok(functions) => {
            if functions.is_empty() {
                Ok("No functions found in the file.".to_string())
            } else {
                Ok(format!("Extracted functions:\n{}", functions.join("\n")))
            }
        }
        Err(err) => Ok(format!("Error parsing Rust file: {}", err)),
    }
}
