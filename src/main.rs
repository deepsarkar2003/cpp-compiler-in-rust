mod codegen;
mod ir;
mod lexer;
mod linker;
mod parser;
mod semantic;
mod server;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cpp_compiler <file_path>");
        return;
    }

    let file_path = &args[1];
    let source_code = match fs::read_to_string(file_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read the file: {}", e);
            return;
        }
    };

    let lexer = lexer::Lexer::new(&source_code);
    let mut parser = parser::Parser::new(lexer);

    match parser.parse() {
        Ok(ast) => {
            if let Err(e) = semantic::SemanticAnalyzer::analyze(&ast) {
                eprintln!("Semantic error: {}", e);
                return;
            }

            let ir = ir::IRGenerator::generate(&ast);
            let machine_code = codegen::CodeGenerator::generate(&ir);
            let executable = linker::Linker::link(vec![machine_code]);

            println!("Executable size: {}", executable.len());

            if let Err(e) = fs::write("output.bin", &executable) {
                eprintln!("Failed to write the executable: {}", e);
            } else {
                println!("Compilation successful! Executable saved as 'output.bin'.");
            }
        }
        Err(e) => {
            eprintln!("Parse error: {}", e);
        }
    }
}
