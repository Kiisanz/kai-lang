# Kai-lang

[![License MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Development Status](https://img.shields.io/badge/status-experimental-orange)](./README.md)
[![Roadmap](https://img.shields.io/badge/roadmap-on--track-blue)](./README.md)

Kai-lang is an experimental statically typed language designed for clarity, concurrency, and embedded DSLs. Its name reflects **breadth (Ocean 海)** and **openness (Beginning 开)** in language design.

## Overview

Kai-lang combines strong static typing with optional type inference, structured concurrency primitives (`async`, `spawn`, `par`), and first-class DSL integration. Syntax is designed to be clean, expressive, and maintainable, enabling both general-purpose programming and domain-specific extensions.

## Parser Status

The AST parser is under active development. Variable declarations (VarDecl) are stable and fully functional. Function declarations (FnDecl) are still being stabilized and may undergo breaking changes. Full AST design, semantic analysis, IR, code generation, and runtime support are in progress.

## Roadmap Highlights

* **Grammar & Lexer** Completed (Sep 2025)
* **Parser Basic AST** In development (Sep 2025 - Nov 2025)
* **Semantic Analysis** Planned (Dec 2025 – Jan 2026)
* **IR & Code Generation** Planned (Feb–Apr 2026)
* **Minimal Runtime & Tooling** Planned (May–Jun 2026)
* **Package Manager Dock** Planned (Jul 2026)
  
## Tooling Ecosystem

* **Kaido** Compiler (Translates .kai source files into native executables)
* **Dock** Package manager
* **Tide** Code formatter
* **Compass** Linter
* **Lantern** Language server
* **Buoy** Testing framework
* **KaiDoc** Documentation generator

## Example

```kai
use std.io

public fn main() -> int32 {
    let msg = "Hello, Kai-lang!"
    io.println(msg)
    return 0
}

html {
    <h1>${"Hello, Kai-lang!"}</h1>
}
```

## License

MIT License ([./LICENSE](./LICENSE))
