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

    if args.to_morse {
        // Convertir texto a morse
        translate_text_to_morse(&input, args.verbose);
    } else {
        // Convertir morse a texto (default)
        translate_morse_to_text(&input, args.verbose);
    }
}

fn translate_text_to_morse(input: &str, verbose: bool) {
    if verbose {
        println!("📝 Input (Text): {}", input);
    }

    let morse_output = semantic::text_to_morse(input);

    if verbose {
        println!("✨ Morse output: {}", morse_output);
    }

    println!("✅ Morse: {}", morse_output);
}

fn translate_morse_to_text(input: &str, verbose: bool) {
    if verbose {
        println!("📝 Input (Morse): {}", input);
    }

    // Lexer
    let tokens: Vec<Token> = Token::lexer(input)
        .filter_map(|result| result.ok())
        .collect();

    if verbose {
        println!("🔤 Tokens: {:?}", tokens);
    }

    // Parser
    let parsed = parser::parse(tokens);

    if verbose {
        println!("📦 Parsed: {:?}", parsed);
    }

    // Semántico
    let letters = semantic::validate_and_translate(parsed);

    if verbose {
        println!("✨ Letters: {:?}", letters);
    }

    // IR
    let ir = ir::IR { letters };

    if verbose {
        println!("🔧 IR: {:?}", ir);
    }

    // Codegen
    let output = codegen::generate(ir);

    println!("✅ Text: {}", output);
}
