# Changelog

All notable changes to the Kalawang programming language project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-08-06

### Added
- **Rust Engine Core**: Fully replaced the legacy Python prototype (`Sawa`) with a performance-focused interpreter written in Rust (`Kalawang`).
- **Unicode Lexer**: Tokenizer with full support for UTF-8 and the Baybayin Unicode block (`U+1700` .. `U+171F`).
- **Dual Syntax & Keywords**: Native recognition for both Tagalog Latin keywords and authentic Baybayin script symbols:
  - **Output / Printing**: `sabihin`, `ipaliwanag`, `print`, `ᜐᜊᜒᜑᜒᜈ᜔`, `ᜁᜉ᜔ᜎᜒᜄ᜔`
  - **Variable Declarations**: `si`, `ipangalan`, `var`, `ᜐᜒ`, `ᜁᜉᜅ᜔ᜎᜈ᜔`
  - **Conditionals**: `kung` / `ᜃᜓᜅ᜔` (if), `okaya` / `ukaya` / `ᜂᜃᜌ` (else if), `kundi` / `ᜃᜓᜈ᜔ᜇᜒ` (else)
  - **Loops**: `habang` / `ᜑᜊ᜔` (while loop)
  - **Booleans**: `tama` / `ᜆᜋ` (`true`), `mali` / `ᜋᜎᜒ` (`false`)
  - **Return**: `ibalik` / `ᜁᜊᜎᜒᜃ᜔`
- **Baybayin Punctuation**:
  - Single Danda (`᜵` - `U+1735`) statement terminator.
  - Double Danda (`᜶` - `U+1736`) section/statement terminator.
- **Abstract Syntax Tree (AST) & Parser**: Recursive descent parser supporting arithmetic, comparisons, variable assignments, blocks, and control flows.
- **Environment & Execution**: Dynamic typing with scoped variable environments and runtime evaluation.
- **Command Line Interface (CLI)**: CLI runner accepting `.bay` and `.kl` files via `kalawang <file>`.
- **Sample Code Scripts**: Included starter example scripts in `examples/`:
  - `hello.bay` (Hello World demonstration in both Latin & Baybayin script)
  - `variables.bay` (Variable declarations & output)
  - `math.bay` (Arithmetic, string concatenation, and nested conditionals)

### Changed
- Shifted language design away from space-sensitive syntax (Python style) to block and danda statement structured syntax for ease of use.
- Standardized file extensions to `.bay` and `.kl`.

---

## [0.0.1] - 2023-09-22

### Added
- Initial experimental prototype named `Sawa` built using Python transliteration.
- Basic proof-of-concept for Baybayin word mapping.
