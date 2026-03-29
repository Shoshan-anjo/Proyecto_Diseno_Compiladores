# 📡 Traductor - Bidirectional Morse Code Translator

A Rust-based compiler that translates between **Morse code and text** bidirectionally. This project demonstrates a complete compilation pipeline with lexical analysis, parsing, semantic analysis, intermediate representation, and code generation.

## 🏗️ Architecture

```
Text → Encoder → Morse        (Text to Morse)
Morse → Lexer → Parser → Semantic Analyzer → IR → Codegen → Text  (Morse to Text)
```

### Modules

- **lexer** (`src/lexer/`) - Tokenizes Morse code input using the `logos` crate
  - `token.rs` - Token enumeration (Dot, Dash, Space)
  
- **parser** (`src/parser/`) - Builds Morse sequences from tokens
  - `core.rs` - Converts token stream into Morse code words

- **semantic** (`src/semantic/`) - Validates, translates and encodes
  - `analyzer.rs` - Maps Morse patterns to ASCII characters (Morse → Text)
  - `encoder.rs` - Maps ASCII characters to Morse patterns (Text → Morse)

- **ir** (`src/ir/`) - Intermediate Representation
  - `representation.rs` - Simple IR structure for character sequences

- **codegen** (`src/codegen/`) - Code generation
  - `translator.rs` - Converts IR to final text output

- **ast** (`src/ast/`) - Abstract Syntax Tree structures
  - `nodes.rs` - AST node definitions

- **cli** (`src/cli.rs`) - Command-line interface using `clap`

- **utils** (`src/utils/`) - Utility functions
  - `error.rs` - Error types and result handling

## 🚀 Usage

### Morse to Text (Default)
```bash
# Default SOS
./traductor.exe
# Output: ✅ Text: SOS

# Custom morse code
./traductor.exe "..." "---" "..."
# Output: ✅ Text: SOS

# With verbose output
./traductor.exe --verbose ".-" "--"
# 📝 Input (Morse): .- --
# 🔤 Tokens: [Dot, Dash, Space, Dash, Dash]
# 📦 Parsed: [".-", "--"]
# ✨ Letters: ['A', 'M']
# 🔧 IR: IR { letters: ['A', 'M'] }
# ✅ Text: AM
```

### Text to Morse
```bash
# Simple word
./traductor.exe --to-morse Hello
# Output: ✅ Morse: .... . .-.. .-.. ---

# Full phrase with verbose
./traductor.exe --to-morse --verbose "Hello World"
# 📝 Input (Text): Hello World
# ✨ Morse output: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
# ✅ Morse: .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

## 📖 Supported Characters

### Letters (A-Z)
```
A: .-     B: -...   C: -.-.   D: -..    E: .      F: ..-.
G: --.    H: ....   I: ..     J: .---   K: -.-    L: .-..
M: --     N: -.     O: ---    P: .--.   Q: --.-   R: .-.
S: ...    T: -      U: ..-    V: ...-   W: .--    X: -..-
Y: -.--   Z: --..
```

### Numbers (0-9)
```
0: -----  1: .----  2: ..---  3: ...--  4: ....-
5: .....  6: -....  7: --...  8: ---..  9: ----.
```

### Special Characters
Supports: `.`, `,`, `?`, `'`, `!`, `/`, `(`, `)`, `&`, `:`, `;`, `=`, `+`, `-`, `_`, `"`, `$`, `@`

## ✅ Features

- ✨ **Bidirectional translation** - Morse ↔ Text
- 🧪 **Unit tests** - 12 comprehensive tests
- 📝 **Verbose mode** - Debug each compilation step
- 🎯 **Complete alphabet** - A-Z, 0-9, special characters
- 🚨 **Error handling** - Graceful unknown character handling
- 📦 **CLI interface** - Easy-to-use command-line arguments
- 🏗️ **Clean architecture** - Modular design with clear separation

## 🧪 Testing

Run all tests:
```bash
cargo test
```

Test output:
```
running 12 tests
test ast::nodes::tests::test_letter_creation ... ok
test ast::nodes::tests::test_word_creation ... ok
test codegen::translator::tests::test_code_generation ... ok
test parser::core::tests::test_parse_morse ... ok
test semantic::analyzer::tests::test_morse_to_letter ... ok
test semantic::analyzer::tests::test_unknown_morse_code ... ok
test semantic::encoder::tests::test_single_letter ... ok
test semantic::encoder::tests::test_simple_word ... ok
test semantic::encoder::tests::test_number ... ok
test semantic::encoder::tests::test_lowercase_conversion ... ok
test semantic::encoder::tests::test_word_with_space ... ok
test semantic::encoder::tests::test_sentence_with_spaces ... ok

test result: ok. 12 passed; 0 failed
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
│   ├── main.rs                    # Entry point with bidirectional logic
│   ├── cli.rs                     # CLI argument parsing
│   ├── lexer/
│   │   ├── mod.rs
│   │   └── token.rs              # Morse token definitions
│   ├── parser/
│   │   ├── mod.rs
│   │   └── core.rs               # Morse sequence parsing
│   ├── semantic/
│   │   ├── mod.rs
│   │   ├── analyzer.rs           # Morse → Text translation
│   │   └── encoder.rs            # Text → Morse translation (NEW)
│   ├── ir/
│   │   ├── mod.rs
│   │   └── representation.rs      # Intermediate representation
│   ├── codegen/
│   │   ├── mod.rs
│   │   └── translator.rs         # Final code generation
│   ├── ast/
│   │   ├── mod.rs
│   │   └── nodes.rs              # AST node definitions
│   └── utils/
│       ├── mod.rs
│       └── error.rs              # Error handling
└── target/                        # Build artifacts
```

## 🎓 Educational Value

This project demonstrates:
- **Compiler architecture** - Complete pipeline from input to output
- **Modular Rust design** - Clean separation of concerns
- **Bidirectional translation** - Two-way conversion logic
- **Pattern matching** - Morse code to character mapping
- **CLI applications** - Modern argument parsing with clap
- **Test-driven development** - Comprehensive test coverage
- **Error handling** - Graceful degradation for unknown inputs

## ⚠️ Limitations

- ✓ ~~Text to Morse translation~~ - **IMPLEMENTED**
- No file input/output support yet
- No multi-language support
- No audio/visual Morse code generation

## 🔮 Future Enhancements

- [ ] File input/output (`--input-file`, `--output-file`)
- [ ] Audio Morse code playback
- [ ] Interactive REPL mode
- [ ] Performance optimizations (caching)
- [ ] WebAssembly compilation
- [ ] Multi-language support
- [ ] Streaming input support
- [ ] Unicode character support

## 🚀 Quick Start

```bash
# Clone and build
git clone https://github.com/Shoshan-anjo/Proyecto_Diseno_Compiladores.git
cd Traductor_Lexer
cargo build --release

# Morse to Text (default)
./target/release/traductor.exe "... --- ..."
# Output: ✅ Text: SOS

# Text to Morse
./target/release/traductor.exe --to-morse "HELLO"
# Output: ✅ Morse: .... . .-.. .-.. ---

# With verbose output
./target/release/traductor.exe --verbose --to-morse "HI"
```

## 📝 License

This project is part of an academic compiler design course.

---

**Made with ❤️ in Rust** | Bidirectional Morse Translator v0.2.0
