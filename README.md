<p align="center">
  <img src="rilologo.png" alt="Rilo logo" width="600"/>
</p>

<h1 align="center">Rilo</h1>

<p align="center">
  A minimal, statically-typed, expression-based programming language that compiles to Z80 assembly for the TI-84+ calculator.
</p>

---

## 🧠 About

**Rilo** is a toy programming language with a C-like syntax, built for fun, learning, and retro hardware. It is designed to be:

- 🧾 **Statically typed** (types checked at compile time)  
- 📦 **Immutable by default** (like Rust or Elm)  
- 📐 **Block-structured** (no inline statements)  
- 🔁 **Supports higher-order functions**  
- 💾 Compiles to **Z80 assembly** (for TI-84+ games and programs)  

This project is being developed using **test-driven development** and written in **Rust**.

---

## 🧱 Language Example

```rilo
fn main() -> int {
    let x = 5;
    return x + 1;
}
```

---

## 🔨 Project Structure

```text
Rilo/
├── src/
│   ├── ast.rs         # AST node definitions
│   ├── lexer.rs       # Tokenizer / lexer
│   ├── parser.rs      # Recursive descent parser
│   ├── token.rs       # Token enum
│   ├── main.rs        # CLI driver
│   └── lib.rs         # Module re-exports
├── tests/
│   └── parser.rs      # Parser unit + integration tests
├── examples/
│   └── demo.rilo      # Example Rilo program
├── rilologo.png       # Project banner/logo
├── Cargo.toml
└── README.md
```

---

## ✅ Features Completed

- [x] Lexer  
- [x] Parser  
- [x] AST  
- [x] Unit tests  
- [x] Test-driven development  
- [ ] Operator precedence  
- [ ] Z80 code generation  
- [ ] Function parameters  
- [ ] Type checker  
- [ ] Error reporting  
