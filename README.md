# Kai-lang🌊

Kai-lang is an experimental programming language designed for clarity, concurrency, and flexible DSL integration.  
The name "Kai" carries multiple meanings: **ocean (海)**, representing breadth and depth, and **beginning/open (开)**, symbolizing openness and exploration in programming design.

---

## Philosophy

- **Wide as the ocean** → flexible and expressive syntax  
- **Open** → welcoming experimentation and DSL embedding  
- **Key** → enabling new paradigms and ideas in software development  

---

## Design Goals

- Strong static typing with optional type inference  
- Concurrency primitives: `async`, `spawn`, `par`  
- First-class DSL blocks: inline SQL, HTML, Regex, and others  
- Clean and readable syntax  

---

## File Extension

- Source files use the `.kai` extension  

---

## Development Status

**Completed**
- Grammar draft written in EBNF  
- Lexer prototype (`.kai` → tokens)  

**In Progress**
- Parser (basic AST, variable declaration parsing, unstable)  

**Not Yet Implemented**
- Full AST design  
- Semantic analysis (type checking, visibility, concurrency rules)  
- Intermediate Representation (IR)  
- Code generation (LLVM backend or VM bytecode)  
- Minimal runtime (async/concurrency)  
- REPL and developer tooling  
- Package manager  

---

## Tentative Roadmap

| Phase                  | Timeline              |
|-------------------------|----------------------|
| Grammar & Lexer         | Sep–Oct 2025         |
| Parser (basic AST)      | Nov 2025             |
| Semantic Analysis       | Dec 2025 – Jan 2026  |
| IR Design               | Feb 2026             |
| Code Generation         | Mar–Apr 2026         |
| Minimal Runtime         | May 2026             |
| REPL + Tooling          | Jun 2026             |
| Package Manager (Dock)  | Jul 2026             |

---

## Tooling Ecosystem

- **Compiler** → Kaido: source code → executable  
- **Package Manager** → Dock: dependency management  
- **Formatter** → Tide: code formatting  
- **Linter** → Compass: code style enforcement  
- **Language Server** → Lantern: editor/IDE integration  
- **Testing Framework** → Buoy: test framework  
- **Documentation Generator** → KaiDoc: code documentation  

---

## Example

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
````

---

## Variable Declaration Syntax

```kai
let <name> = <expr>;          // inferred type
mut <name> = <expr>;          // mutable variable
public let <name> = <expr>;   // public variable
private let <name> = <expr>;  // private variable
let x: int32 = 10;            // explicit type
```

---

## Operators

**Unary Operators**

```kai
-x    // negation
+x    // unary plus
!x    // logical NOT

// Chaining supported in future
let a = -!+x;
```

**Binary Operators**

```kai
x + y
x - y
x * y
x / y
x % y
```

* Operator precedence: `* / %` > `+ -`

---

## License
Currently unlicensed. A license will be chosen once Kai-lang reaches prototype stability.

