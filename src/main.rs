mod codegen;
mod ir;
mod lexer;
mod parser;
mod semantic;

mod ast;
mod cli;
mod interactive;
pub mod utils;

use logos::Logos;
use lexer::Token;
use cli::Args;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let input = args.input.join(" ");

    // Si no hay entrada, usar modo interactivo
    if input.trim().is_empty() && !args.to_morse && !args.verbose {
        interactive::run_interactive_mode();
        return;
    }

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
        println!("📝 Entrada (Texto): {}", input);
    }

    let morse_output = semantic::text_to_morse(input);

    if verbose {
        println!("✨ Salida Morse: {}", morse_output);
    }

    println!("✅ Morse: {}", morse_output);
}

fn translate_morse_to_text(input: &str, verbose: bool) {
    if verbose {
        println!("📝 Entrada (Morse): {}", input);
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
        println!("📦 Parseado: {:?}", parsed);
    }

    // Semántico
    let letters = semantic::validate_and_translate(parsed);

    if verbose {
        println!("✨ Letras: {:?}", letters);
    }

    // IR
    let ir = ir::IR { letters };

    if verbose {
        println!("🔧 IR: {:?}", ir);
    }

    // Codegen
    let output = codegen::generate(ir);

    println!("✅ Texto: {}", output);
}
