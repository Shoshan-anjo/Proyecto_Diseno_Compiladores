# 📡 Traductor - Traductor Bidireccional de Código Morse

Un compilador basado en Rust que traduce entre **código Morse y texto** bidireccionalemente. Este proyecto demuestra un pipeline completo de compilación con análisis léxico, análisis sintáctico, análisis semántico, representación intermedia y generación de código.

## 🏗️ Arquitectura

```
Texto → Codificador → Morse        (Texto a Morse)
Morse → Lexer → Parser → Analizador Semántico → IR → Codegen → Texto  (Morse a Texto)
```

### Módulos

- **lexer** (`src/lexer/`) - Tokeniza entrada de código Morse usando la caja `logos`
  - `token.rs` - Enumeración de tokens (Punto, Guión, Espacio)
  
- **parser** (`src/parser/`) - Construye secuencias de Morse desde tokens
  - `core.rs` - Convierte flujo de tokens en palabras de código Morse

- **semantic** (`src/semantic/`) - Valida, traduce y codifica
  - `analyzer.rs` - Mapea patrones Morse a caracteres ASCII (Morse → Texto)
  - `encoder.rs` - Mapea caracteres ASCII a patrones Morse (Texto → Morse)

- **ir** (`src/ir/`) - Representación Intermedia
  - `representation.rs` - Estructura IR simple para secuencias de caracteres

- **codegen** (`src/codegen/`) - Generación de código
  - `translator.rs` - Convierte IR a salida de texto final

- **ast** (`src/ast/`) - Estructuras del Árbol de Sintaxis Abstracta
  - `nodes.rs` - Definiciones de nodos AST

- **cli** (`src/cli.rs`) - Interfaz de línea de comandos usando `clap`

- **utils** (`src/utils/`) - Funciones de utilidad
  - `error.rs` - Tipos de errores y manejo de resultados

## 🚀 Uso

### Morse a Texto (Predeterminado)
```bash
# SOS predeterminado
./traductor.exe
# Salida: ✅ Texto: SOS

# Código morse personalizado
./traductor.exe "..." "---" "..."
# Salida: ✅ Texto: SOS

# Con salida detallada
./traductor.exe --verbose ".-" "--"
# 📝 Entrada (Morse): .- --
# 🔤 Tokens: [Punto, Guión, Espacio, Guión, Guión]
# 📦 Parseado: [".-", "--"]
# ✨ Letras: ['A', 'M']
# 🔧 IR: IR { letras: ['A', 'M'] }
# ✅ Texto: AM
```

### Texto a Morse
```bash
# Palabra simple
./traductor.exe --to-morse mama
# Salida: ✅ Morse: -- .- -- .-

# Frase completa con salida detallada
./traductor.exe --to-morse --verbose "Hola Mundo"
# 📝 Entrada (Texto): Hola Mundo
# ✨ Salida Morse: .... --- .-.. .- / -- ..- -. -.. ---
# ✅ Morse: .... --- .-.. .- / -- ..- -. -.. ---
```

## 📖 Caracteres Soportados

### Letras (A-Z)
```
A: .-     B: -...   C: -.-.   D: -..    E: .      F: ..-.
G: --.    H: ....   I: ..     J: .---   K: -.-    L: .-..
M: --     N: -.     O: ---    P: .--.   Q: --.-   R: .-.
S: ...    T: -      U: ..-    V: ...-   W: .--    X: -..-
Y: -.--   Z: --..
```

### Números (0-9)
```
0: -----  1: .----  2: ..---  3: ...--  4: ....-
5: .....  6: -....  7: --...  8: ---..  9: ----.
```

### Caracteres Especiales
Soporta: `.`, `,`, `?`, `'`, `!`, `/`, `(`, `)`, `&`, `:`, `;`, `=`, `+`, `-`, `_`, `"`, `$`, `@`

## ✅ Características

- ✨ **Traducción bidireccional** - Morse ↔ Texto
- 🧪 **Pruebas unitarias** - 12 pruebas completas
- 📝 **Modo detallado** - Depura cada paso de compilación
- 🎯 **Alfabeto completo** - A-Z, 0-9, caracteres especiales
- 🚨 **Manejo de errores** - Gestión elegante de caracteres desconocidos
- 📦 **Interfaz CLI** - Argumentos de línea de comandos fáciles de usar
- 🏗️ **Arquitectura limpia** - Diseño modular con separación clara

## 🧪 Pruebas

Ejecuta todas las pruebas:
```bash
cargo test
```

Salida de pruebas:
```
ejecutando 12 pruebas
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

resultado de prueba: ok. 12 pasadas; 0 fallidas
```

## 📦 Dependencias

- **logos** (0.16.1) - Análisis léxico
- **clap** (4.0) - Análisis de línea de comandos

## 🏗️ Construcción

```bash
# Desarrollo
cargo build

# Versión optimizada
cargo build --release
```

## 📋 Estructura del Proyecto

```
Traductor_Lexer/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs                    # Punto de entrada con lógica bidireccional
│   ├── cli.rs                     # Análisis de argumentos CLI
│   ├── lexer/
│   │   ├── mod.rs
│   │   └── token.rs              # Definiciones de tokens Morse
│   ├── parser/
│   │   ├── mod.rs
│   │   └── core.rs               # Análisis de secuencias Morse
│   ├── semantic/
│   │   ├── mod.rs
│   │   ├── analyzer.rs           # Traducción Morse → Texto
│   │   └── encoder.rs            # Traducción Texto → Morse (NUEVO)
│   ├── ir/
│   │   ├── mod.rs
│   │   └── representation.rs      # Representación intermedia
│   ├── codegen/
│   │   ├── mod.rs
│   │   └── translator.rs         # Generación de código final
│   ├── ast/
│   │   ├── mod.rs
│   │   └── nodes.rs              # Definiciones de nodos AST
│   └── utils/
│       ├── mod.rs
│       └── error.rs              # Manejo de errores
└── target/                        # Artefactos de construcción
```

## 🎓 Valor Educativo

Este proyecto demuestra:
- **Arquitectura de compiladores** - Pipeline completo de entrada a salida
- **Diseño modular en Rust** - Separación clara de responsabilidades
- **Traducción bidireccional** - Lógica de conversión en dos direcciones
- **Coincidencia de patrones** - Mapeo de código Morse a caracteres
- **Aplicaciones CLI** - Análisis moderno de argumentos con clap
- **Desarrollo dirigido por pruebas** - Cobertura completa de pruebas
- **Manejo de errores** - Degradación elegante para entradas desconocidas

## ⚠️ Limitaciones

- ✓ ~~Traducción de Texto a Morse~~ - **IMPLEMENTADO**
- Sin soporte de entrada/salida de archivos aún
- Sin soporte multilingüe
- Sin generación de código Morse de audio/visual

## 🔮 Mejoras Futuras

- [ ] Entrada/salida de archivos (`--input-file`, `--output-file`)
- [ ] Reproducción de código Morse de audio
- [ ] Modo REPL interactivo
- [ ] Optimizaciones de rendimiento (caché)
- [ ] Compilación a WebAssembly
- [ ] Soporte multilingüe
- [ ] Soporte de entrada con streaming
- [ ] Soporte de caracteres Unicode

## 🚀 Inicio Rápido

```bash
# Clonar y construir
git clone https://github.com/Shoshan-anjo/Proyecto_Diseno_Compiladores.git
cd Traductor_Lexer
cargo build --release

# Morse a Texto (predeterminado)
./target/release/traductor.exe "... --- ..."
# Salida: ✅ Texto: SOS

# Texto a Morse
./target/release/traductor.exe --to-morse "HOLA"
# Salida: ✅ Morse: .... --- .-.. .-

# Con salida detallada
./target/release/traductor.exe --verbose --to-morse "HI"
```

## 📝 Licencia

Este proyecto es parte de un curso académico de diseño de compiladores.

---

**Hecho con ❤️ en Rust** | Traductor Bidireccional de Morse v0.2.0
