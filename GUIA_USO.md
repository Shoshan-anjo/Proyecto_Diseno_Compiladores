# 🚀 Cómo usar el Traductor Morse - Guía Completa

## 📋 Dos Formas de Usar el Traductor

### OPCIÓN 1: MODO INTERACTIVO (Recomendado para usuarios)

Ejecuta el programa sin argumentos:

```bash
./traductor.exe
```

O con cargo:

```bash
cargo run --release
```

**Verás un menú como este:**

```
╔════════════════════════════════════════╗
║  📡 TRADUCTOR MORSE BIDIRECCIONAL    ║
║  Modo Interactivo                      ║
╚════════════════════════════════════════╝

¿Qué deseas hacer?
1. Convertir Morse → Texto
2. Convertir Texto → Morse
3. Salir

Elige una opción (1-3): 
```

**Ejemplo 1: Morse a Texto**

```
Elige una opción (1-3): 1
📡 Morse a Texto
─────────────────
Ingresa el código Morse (separa letras con espacios): ... --- ...

✅ Resultado: SOS
```

**Ejemplo 2: Texto a Morse**

```
Elige una opción (1-3): 2
✉️  Texto a Morse
─────────────────
Ingresa el texto a convertir: mama

✅ Resultado: -- .- -- .-
```

**Ejemplo 3: Salir**

```
Elige una opción (1-3): 3

¡Hasta luego! 👋
```

---

### OPCIÓN 2: MODO LÍNEA DE COMANDOS (Para expertos/scripts)

Si prefieres pasar argumentos directamente:

#### **Convertir Morse a Texto**

```bash
./traductor.exe "... --- ..."
# ✅ Texto: SOS

./traductor.exe "...." "." ".-.." ".-.." "---"
# ✅ Texto: HELLO
```

#### **Convertir Texto a Morse**./traductor.exe --to-morse mama

#### **Modo Detallado (Verbose)**

```bash
./traductor.exe --to-morse --verbose "HI"

# 📝 Entrada (Texto): HI
# ✨ Salida Morse: .... ..
# ✅ Morse: .... ..
```

---

## 🔧 Banderas Disponibles

```
BANDERAS:
  -t, --to-morse    Traduce TEXTO a MORSE (por defecto: MORSE a TEXTO)
  -v, --verbose     Muestra detalles de cada paso del proceso
  -h, --help        Muestra esta ayuda
```

---

## 📚 Ejemplos Prácticos

### Conversiones Morse → Texto

```bash
# SOS (Señal de auxilio)
./traductor.exe "... --- ..."
# ✅ Texto: SOS

# HOLA
./traductor.exe ".... --- .-.. .-"
# ✅ Texto: HOLA

# COMPILADOR
./traductor.exe "-.-. --- -- .--. .. .-.. .- -.. --- .-."
# ✅ Texto: COMPILADOR
```

### Conversiones Texto → Morse

```bash
# mama
./traductor.exe --to-morse mama
# ✅ Morse: -- .- -- .-

# RUST
./traductor.exe --to-morse RUST
# ✅ Morse: .-. ..- ... -

# TRADUCTOR
./traductor.exe --to-morse traductor
# ✅ Morse: - .-. .- -.. ..- -.-. - --- .-.
```

---

## 💡 Tips Útiles

### Para Usuario Normal

- **Usa modo interactivo** (`./traductor.exe` sin argumentos)
- Más fácil, menús claros, manejo de errores visible

### Para Desarrollador/Scripts

- **Usa CLI directo** con argumentos
- Perfecto para automatizar procesos
- Fácil de integrar en otros scripts

### Para Debugging

- **Usa --verbose** para ver cada paso
- Útil para entender cómo funciona el compilador
- Ver tokens, parsing, IR, etc.

---

## 🎯 Comparación de Opciones

| Característica           | Interactivo | CLI            |
| ------------------------- | ----------- | -------------- |
| **Fácil de usar**  | ✅ Sí      | No             |
| **Rápido**         | Normal      | ✅ Muy rápido |
| **Para scripts**    | No          | ✅ Sí         |
| **Menús visuales** | ✅ Sí      | No             |
| **Entrada manual**  | ✅ Sí      | No             |
| **Automatizable**   | No          | ✅ Sí         |

---

## ❓ Preguntas Frecuentes

**P: ¿Por qué obtengo "opción inválida"?**
R: Solo puedes ingresar 1, 2 o 3 en el menú interactivo.

**P: ¿Puedo usar espacios en Morse?**
R: Sí, por defecto separa caracteres con espacios.

**P: ¿Qué pasa si ingreso un morse inválido?**
R: Se mostrará "?" para los códigos no reconocidos.

**P: ¿Es sensible a mayúsculas/minúsculas?**
R: No, todo se convierte a mayúsculas automáticamente.

---

## 🚀 Próximos Pasos

El proyecto sigue en desarrollo. Próximas características:

- [ ] Entrada/salida de archivos
- [ ] Interfaz gráfica
- [ ] Reproducción de audio
- [ ] Más idiomas

---

**¡Disfruta del Traductor Morse! 📡**
