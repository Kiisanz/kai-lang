# Changelog

All notable changes to Kai-lang will be documented here

## [Unreleased]

### Planned v0.2.0

* Semantic analysis
* Type checking
* Variable scope and name resolution

## [0.1.0] Initial Release

### Lexer

* Full token set including literals keywords symbols and operators
* Multi character tokens such as -> == != <= >= && || ${
* Support for string integer float and boolean literals
* Keyword recognition including let var fn return type use public async spawn par dsl if else
* Comment skipping using //
* Accurate span tracking with line and column

### Parser

* Variable declarations using let for immutable and var for mutable
* Variable assignment
* Type annotations with optional inference
* Type system covering named types array types such as User[] generic types such as Result<T, E> and optional types such as T?
* Function declarations with typed parameters and optional return type
* Support for public fn
* Return statement
* Use statement with dot separated paths such as use std.io
* If and if else statements
* Binary expressions with correct precedence
* Dot access and method calls such as io.println(msg)
* Function calls with arguments

### Testing

* 18 tests passing across lexer and parser

