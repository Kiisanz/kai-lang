# Kai-lang 🌊

**Kai-lang** is an experimental programming language in the early research and prototyping stage.

The name **Kai** carries layered meaning:

* **Japanese (海 / kai)** → ocean — wide, flexible, open.
* **Chinese (开 / kai)** → open, key, beginning.
* Combined with **lang** → makes clear this is a programming language.

**Philosophy:** a language that is **wide as the ocean, open to exploration, and a key to new ideas.**

---

## ✨ Design Goals

Kai-lang aims to combine:

* **Clarity of syntax** with **strong static typing** (with optional inference).
* **Modern concurrency primitives**: `async`, `spawn`, `par`.
* **First-class DSL blocks**: inline SQL, HTML, Regex, etc.
* **Clean, familiar syntax** influenced by Rust, Kotlin, and TypeScript.

---

## 🏗️ Development Status

Kai-lang is at an **early experimental** stage.

### ✅ Completed

* Grammar draft written in EBNF.
* Lexer prototype (`.kai` → tokens).

### 🚧 In Progress

* Parser (currently experimenting with **variable declaration parsing**, unstable).

### ❌ Not Yet Implemented

* Full AST design.
* Semantic analysis (type checking, visibility, concurrency rules).
* Intermediate Representation (IR).
* Code generation (LLVM backend or VM bytecode).
* Minimal runtime (async/concurrency).
* REPL and developer tooling.
* Package manager.

---

## 📅 Tentative Roadmap

* **Grammar & Lexer** → Sep–Oct 2025
* **Parser (basic AST)** → Nov 2025 *(current focus: VarDecl parser, unstable)*
* **Semantic Analysis** → Dec 2025 – Jan 2026
* **IR Design** → Feb 2026
* **Code Generation (LLVM / VM)** → Mar–Apr 2026
* **Minimal Runtime** → May 2026
* **REPL + Tooling** → Jun 2026
* **Package Manager (Dock)** → Jul 2026

---

## ⚙️ Tooling Ecosystem

* **Compiler** → **Kaido** 🛤️
  The “sea route” from source code to executable.

* **Package Manager** → **Dock** ⚓
  A harbor for dependencies.

* **Formatter** → **Tide** 🌊
  Keeps code clean and consistent.

* **Linter** → **Compass** 🧭
  Ensures code stays on course.

* **Language Server** → **Lantern** 🏮
  Lights the way inside editors and IDEs.

* **Testing** → **Buoy** ⛵
  Keeps your code afloat.

* **Documentation Generator** → **KaiDoc** 📖
  A map for navigating your codebase.

---

## 📦 File Extension

Kai-lang source files use:

```
.kai
```

---

## 🚀 Example (Design Draft)

```kai
use std.io;

public fn main() -> int32 {
    let msg = "Hello, Kai-lang!";
    io.println(msg);
    return 0;
}

html {
  <h1>${"Hello, Kai-lang!"}</h1>
}
```

---

## 📜 License

Currently unlicensed. A license will be chosen once Kai-lang reaches prototype stability.
