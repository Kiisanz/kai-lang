# Kai-lang

Kai-lang is an **experimental programming language** currently in the research and prototyping stage. The name **Kai** carries layered meaning:

* In Japanese (*海* / kai): **ocean** — wide, flexible, and open.
* In Chinese (*开* / kai): **open, key** — symbolizing a gateway to something new.
* Combined with **lang**: makes clear that this is a programming language.

Kai-lang is designed to combine clarity of syntax, strong typing, and first-class concurrency while also supporting **embedded DSLs** for tasks like SQL, HTML, or Regex. It is still highly experimental and not yet ready for production.

---

## Development Phase 🚧

**Status:** Early **experimental**

Kai-lang is currently at the very first stages of development. At this point:

* ✅ **Grammar draft** has been written in EBNF to outline the syntax and semantics of the language.
* ✅ **Lexer prototype** is in progress to tokenize Kai source files (`.kai`).
* ❌ **Parser** is not yet implemented.
* ❌ **AST (Abstract Syntax Tree)** design is under discussion.
* ❌ **Semantic analysis** (type checking, ownership rules, concurrency model) is not yet started.
* ❌ **Code generation** (LLVM backend or VM bytecode) is not yet started.
* ❌ **Runtime** is not yet designed.

At this stage, Kai-lang is a **conceptual experiment**. The purpose is to validate the design of the grammar and the feasibility of a modern programming language that combines:

* Strong static typing with optional inference.
* Modern concurrency primitives (`async`, `spawn`, `par`).
* Inline DSL blocks with string interpolation (`sql { ... }`, `html { ... }`, `regex { ... }`).
* A clean syntax influenced by Rust, Kotlin, and TypeScript.

---

## Roadmap with Timeline

The planned phases of Kai-lang development (tentative timeline):

1. **Grammar & Lexer** — define syntax and implement tokenization. *(Sep–Oct 2025)*
2. **Parser** — build an AST from tokens. *(Nov 2025)*
3. **Semantic Analysis** — enforce typing, visibility, concurrency rules. *(Dec 2025–Jan 2026)*
4. **Intermediate Representation** — design IR for optimizations. *(Feb 2026)*
5. **Code Generation** — LLVM backend or custom VM. *(Mar–Apr 2026)*
6. **Basic Runtime** — minimal runtime for async/concurrency. *(May 2026)*
7. **REPL & Tooling** — interactive shell, formatter, language server. *(Jun 2026)*
8. **Package Manager** — dependency management, module system. *(Jul 2026)*

> Note: This timeline is **exploratory** and may shift depending on research outcomes and contributor involvement.

---

## Example (Design Draft)

```kai
use std.io;

public fn main() -> int32 {
    let msg = "Hello, Kai-lang!";
    io.println(msg);
    return 0;
}
```

With DSL block:

```kai
html {
  <h1>${"Hello, Kai-lang!"}</h1>
}
```

---

## File Extension

Kai-lang source files use the extension:

```
.kai
```

---

## License

Currently unlicensed. A license will be chosen once Kai-lang reaches prototype stability.
