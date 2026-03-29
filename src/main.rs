mod codegen;
mod ir;
mod lexer;
mod parser;
mod semantic;

mod ast;
mod cli;
pub mod utils;

use logos::Logos;
use lexer::Token;
use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let input = args.input.join(" ");

    if args.verbose {
        println!("📝 Input: {}", input);
    }

    // Lexer
    let tokens: Vec<Token> = Token::lexer(&input)
        .filter_map(|result| result.ok())
        .collect();

    if args.verbose {
        println!("🔤 Tokens: {:?}", tokens);
    }

    // Parser
    let parsed = parser::parse(tokens);

    if args.verbose {
        println!("📦 Parsed: {:?}", parsed);
    }

    // Semántico
    let letters = semantic::validate_and_translate(parsed);

    if args.verbose {
        println!("✨ Letters: {:?}", letters);
    }

    // IR
    let ir = ir::IR { letters };

    if args.verbose {
        println!("🔧 IR: {:?}", ir);
    }

    // Codegen
    let output = codegen::generate(ir);

    println!("✅ Resultado: {}", output);
}
