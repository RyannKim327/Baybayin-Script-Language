# AGENT.md

## Project

**Project Name:** Kalawang

This project aims to build a modern programming language whose syntax and keywords are written using the Baybayin script while being fully Unicode-compliant.

The first implementation will be an **interpreter written in Rust**. Once the language design stabilizes, it may evolve into a compiler targeting LLVM, WebAssembly, or native machine code.

---

# Vision

Create a programming language that:

* Uses Baybayin keywords.
* Has simple, readable syntax.
* Supports modern programming features.
* Is cross-platform.
* Has excellent error messages.
* Encourages learning programming through Filipino culture.

This project values clarity over cleverness.

---

# Goals

Current goals:

* Read source files.
* Tokenize Baybayin source code.
* Parse tokens into an AST.
* Execute code using an interpreter.
* Add variables, expressions, and functions.
* Build a standard library.
* Eventually support compilation.

---

# Non-Goals

For the first version, do **not** implement:

* JIT compilation
* Garbage collection
* Multithreading
* Macros
* Generics
* Async/await
* Package manager

Focus on creating a solid language core first.

---

# Tech Stack

Language:

* Rust (stable)

Suggested crates:

* logos (lexer)
* chumsky (parser)
* ariadne (error reporting)
* clap (CLI)
* serde (configuration)
* anyhow (application errors)
* thiserror (custom error types)

---

# Repository Structure

```
src/
│
├── main.rs
├── cli.rs
│
├── lexer/
│   ├── mod.rs
│   ├── lexer.rs
│   └── token.rs
│
├── parser/
│   ├── mod.rs
│   ├── parser.rs
│   └── ast.rs
│
├── interpreter/
│   ├── mod.rs
│   ├── environment.rs
│   ├── value.rs
│   └── interpreter.rs
│
├── diagnostics/
│   └── error.rs
│
└── utils/
```

---

# Development Order

## Phase 1

Lexer

Responsibilities:

* Read UTF-8 source
* Recognize Baybayin keywords
* Produce tokens
* Track line and column numbers

---

## Phase 2

Parser

Responsibilities:

* Consume tokens
* Produce an Abstract Syntax Tree
* Report syntax errors clearly

---

## Phase 3

Interpreter

Responsibilities:

* Execute AST nodes
* Evaluate expressions
* Manage variables
* Handle scopes

---

## Phase 4

Language Features

Implement:

* Variables
* Numbers
* Strings
* Booleans
* Arithmetic
* Comparisons
* If statements
* While loops
* Functions
* Return statements

---

## Phase 5

Standard Library

Examples:

* print
* input
* length
* string utilities
* file I/O

---

# Coding Principles

Always:

* Prefer readability over optimization.
* Keep modules small.
* Write idiomatic Rust.
* Avoid unnecessary abstractions.
* Keep functions focused on one responsibility.
* Add tests for new language features.

---

# Error Messages

Error messages should explain:

* What happened.
* Where it happened.
* How to fix it.

Example:

```
Unexpected token "}"

Expected expression before "}"
```

Avoid compiler jargon when a simpler explanation is possible.

---

# Unicode

The language must be Unicode-first.

Requirements:

* Baybayin keywords
* UTF-8 source files
* Unicode identifiers (future)
* Correct character handling

Do not assume one byte equals one character.

---

# AST Philosophy

The AST should represent language concepts, not syntax.

Example:

Expression

* Number
* String
* Variable
* Binary
* Unary
* Call

Statement

* Print
* VariableDeclaration
* Assignment
* If
* While
* Function
* Return

---

# Testing

Every implemented feature should include tests.

Examples:

* Lexer tests
* Parser tests
* Interpreter tests

Regression tests should be added for every fixed bug.

---

# Style Guide

* Use snake_case for functions.
* Use PascalCase for structs and enums.
* Avoid deeply nested matches where simpler patterns exist.
* Document public APIs.
* Run `cargo fmt` and `cargo clippy` before committing.

---

# AI Assistant Guidelines

When contributing code:

1. Preserve the existing architecture.
2. Avoid unnecessary dependencies.
3. Prefer explicit code over metaprogramming.
4. Keep parser, lexer, and interpreter separate.
5. Explain design decisions in comments when they are non-obvious.
6. Do not introduce unfinished features without clear TODOs.

---

# Long-Term Roadmap

Version 0.1

* Lexer
* Parser
* Interpreter

Version 0.2

* Variables
* Functions
* Conditionals

Version 0.3

* Modules
* Standard library

Version 0.4

* Bytecode interpreter (optional)

Version 1.0

* Native compiler
* Package manager
* Documentation
* Language Server Protocol (LSP)
* Visual Studio Code extension

---

# Project Motto

> Build a language that is simple to learn, enjoyable to use, and proudly rooted in Baybayin while following modern language-design practices.
