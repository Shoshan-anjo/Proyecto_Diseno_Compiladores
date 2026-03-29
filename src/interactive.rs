use std::io::{self, Write};

pub fn run_interactive_mode() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║  📡 TRADUCTOR MORSE BIDIRECCIONAL    ║");
    println!("║  Modo Interactivo                      ║");
    println!("╚════════════════════════════════════════╝\n");

    loop {
        println!("\n¿Qué deseas hacer?");
        println!("1. Convertir Morse → Texto");
        println!("2. Convertir Texto → Morse");
        println!("3. Salir");
        print!("\nElige una opción (1-3): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error al leer entrada");

        match choice.trim() {
            "1" => morse_to_text(),
            "2" => text_to_morse(),
            "3" => {
                println!("\n¡Hasta luego! 👋\n");
                break;
            }
            _ => println!("❌ Opción inválida. Por favor elige 1, 2 o 3."),
        }
    }
}

fn morse_to_text() {
    use crate::{lexer::Token, parser, semantic, ir, codegen};
    use logos::Logos;

    println!("\n📡 Morse a Texto");
    println!("─────────────────");
    print!("Ingresa el código Morse (separa letras con espacios): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada");

    let input = input.trim();
    
    if input.is_empty() {
        println!("❌ Por favor ingresa algo válido.\n");
        return;
    }

    // Lexer
    let tokens: Vec<Token> = Token::lexer(input)
        .filter_map(|result| result.ok())
        .collect();

    // Parser
    let parsed = parser::parse(tokens);

    // Semántico
    let letters = semantic::validate_and_translate(parsed);

    // IR
    let ir_result = ir::IR { letters };

    // Codegen
    let output = codegen::generate(ir_result);

    println!("\n✅ Resultado: {}\n", output);
}

fn text_to_morse() {
    use crate::semantic;

    println!("\n✉️  Texto a Morse");
    println!("─────────────────");
    print!("Ingresa el texto a convertir: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada");

    let input = input.trim();
    
    if input.is_empty() {
        println!("❌ Por favor ingresa algo válido.\n");
        return;
    }

    let morse_output = semantic::text_to_morse(input);

    println!("\n✅ Resultado: {}\n", morse_output);
}
