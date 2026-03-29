# 📡 Traductor - Morse Code to Text Translator

A Rust-based compiler that translates Morse code into text. This project demonstrates a complete compilation pipeline with lexical analysis, parsing, semantic analysis, intermediate representation, and code generation.

## 🏗️ Architecture

```
Morse Input → Lexer → Parser → Semantic Analyzer → IR → Codegen → Text Output
```

### Modules

- **lexer** (`src/lexer/`) - Tokenizes Morse code input using the `logos` crate
  - `token.rs` - Token enumeration (Dot, Dash, Space)
  
- **parser** (`src/parser/`) - Builds Morse sequences from tokens
  - `core.rs` - Converts token stream into Morse code words

- **semantic** (`src/semantic/`) - Validates and translates Morse to characters
  - `analyzer.rs` - Maps Morse patterns to ASCII characters with error handling

- **ir** (`src/ir/`) - Intermediate Representation
  - `representation.rs` - Simple IR structure for character sequences

- **codegen** (`src/codegen/`) - Code generation
  - `translator.rs` - Converts IR to final text output

- **ast** (`src/ast/`) - Abstract Syntax Tree structures
  - `node.rs` - AST node definitions

- **cli** (`src/cli.rs`) - Command-line interface using `clap`

- **utils** (`src/utils/`) - Utility functions
  - `error.rs` - Error types and result handling

## 🚀 Usage

### Default (SOS)
```bash
cargo run --release
# Output: ✅ Resultado: SOS
```

### Custom Morse Code
```bash
cargo run --release -- "..." "---" "..."
# Output: ✅ Resultado: SOS
```

### Verbose Output
```bash
cargo run --release -- --verbose "..." "---"
# Output shows each compilation step
```

## 📖 Supported Morse Code

The translator supports:

- **Letters**: A-Z
- **Numbers**: 0-9
- **Special handling**: Unknown codes map to '?'

Examples:
- `...` → S (SOS distress signal)
- `---` → O
- `.-` → A
- `--` → M

## ✅ Features

- ✨ Complete compilation pipeline
- 🧪 Unit tests for each module
- 📝 Verbose output mode for debugging
- 🚨 Error handling for unknown Morse codes
- 🎯 Clean module separation
- 📦 Command-line argument parsing

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Test output:
```
running 5 tests
test ast::node::tests::test_ast_node_creation ... ok
test codegen::translator::tests::test_code_generation ... ok
test parser::core::tests::test_parse_morse ... ok
test semantic::analyzer::tests::test_morse_to_letter ... ok
test semantic::analyzer::tests::test_unknown_morse_code ... ok

test result: ok. 5 passed; 0 failed
```

## 📦 Dependencies

- **logos** (0.16.1) - Lexical analysis
- **clap** (4.0) - Command-line parsing

## 🏗️ Building

```bash
# Development
cargo build

# Release (optimized)
cargo build --release
```

## 📋 Project Structure

```
Traductor_Lexer/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── lexer/
│   │   ├── mod.rs
│   │   └── token.rs
│   ├── parser/
│   │   ├── mod.rs
│   │   └── core.rs
│   ├── semantic/
│   │   ├── mod.rs
│   │   └── analyzer.rs
│   ├── ir/
│   │   ├── mod.rs
│   │   └── representation.rs
│   ├── codegen/
│   │   ├── mod.rs
│   │   └── translator.rs
│   ├── ast/
│   │   ├── mod.rs
│   │   └── node.rs
│   └── utils/
│       ├── mod.rs
│       └── error.rs
└── target/
```

## 🎓 Educational Value

This project demonstrates:
- Compiler design patterns
- Modular Rust architecture
- Trait usage (Logos, Parser, Iterator)
- Error handling strategies
- CLI applications with clap
- Unit testing in Rust

## ⚠️ Limitations

- Currently supports only basic Morse code patterns
- No file input/output support
- Limited to standard Morse characters
- No multi-language support

## 🔮 Future Enhancements

- [ ] File input/output
- [ ] Text to Morse translation (reverse)
- [ ] Additional character support (punctuation, accents)
- [ ] Performance optimizations
- [ ] Interactive REPL mode
- [ ] WebAssembly compilation

---

**Made with ❤️ in Rust**
