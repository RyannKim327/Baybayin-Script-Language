# Changelog

All notable changes to the Kalawang programming language project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-08-08

### Added
- **Arrays & Lists Support (`mga`, `bilang`, `list`, `array`, `[]`)**:
  - **Bracket Literals**: Support for bracket syntax `[elem1, elem2, ...]` including empty lists `[]` and trailing commas.
  - **List Constructors**: Support for `mga(...)`, `bilang(...)`, `list(...)`, `array(...)`, and Baybayin `ᜋ᜔ᜄ(...)` / `ᜊᜒᜎᜅ᜔(...)`.
  - **Indexing & Negative Indexing**: Support for element access `arr[index]` with positive and negative indexing (`arr[-1]` for last element).
  - **Index Assignment**: In-place element modification via `arr[index] = new_value`.
  - **Array Operations**:
    - Concatenation (`+`) between arrays, elements, and formatted string conversion.
    - Repetition (`*`) for repeating array elements.
    - Equality comparison (`==` and `!=`).
  - **Built-in Array Functions**:
    - `haba` / `sukat` / `length` / `len` / `count` / `ᜑᜊ` / `ᜐᜓᜃᜆ᜔` (length of array or string)
    - `dagdag` / `idagdag` / `push` / `append` / `ᜇᜄ᜔ᜇᜄ᜔` / `ᜁᜇᜄ᜔ᜇᜄ᜔` (append item to array)
    - `alis` / `tanggalin` / `pop` / `remove` / `ᜀᜎᜒᜐ᜔` / `ᜆᜅ᜔ᜄᜎᜒᜈ᜔` (remove item from array)
    - `nandyan` / `mayroon` / `meron` / `contains` / `includes` / `ᜈᜈ᜔ᜇ᜔ᜌᜈ᜔` (check element presence)
    - `pagsamahin` / `join` / `ᜉᜄ᜔ᜐᜋᜑᜒᜈ᜔` (join elements into string with delimiter)
    - `baligtad` / `reverse` / `ᜊᜎᜒᜄ᜔ᜆᜇ᜔` (reverse array elements)
  - **Type Conversion**: Conversion support for `"array"` / `"list"` / `"mga"` in `isalin` / `convert`.
  - **Example Scripts**: Added `examples/tagalog/array.bay`, `examples/english/array.bay`, and `examples/baybayin/array.bay`.

## [0.2.0] - 2026-08-07

### Added
- **Multi-Lingual Keyword Support (English, Tagalog & Baybayin)**: Extended keyword recognition across lexer and parser to support English keywords alongside Latin Tagalog and authentic Baybayin script:
  - **Output / Print**: `tell`, `say`, `print` (Tagalog: `sabihin`, `ipaliwanag` | Baybayin: `ᜐᜊᜒᜑᜒᜈ᜔`, `ᜁᜉᜎᜒᜏᜈᜄ᜔`)
  - **Variables**: `that` (Tagalog: `si`, `ipangalan` | Baybayin: `ᜐᜒ`, `ᜁᜉᜅᜎᜈ᜔`)
  - **Conditionals**: `if`, `elseif`, `else` (Tagalog: `kung`, `okaya`/`ukaya`, `kundi` | Baybayin: `ᜃᜓᜅ᜔`, `ᜂᜃᜌ`, `ᜃᜓᜈ᜔ᜇᜒ`)
  - **Loops & Flow**: `while`, `return` (Tagalog: `habang`, `ibalik` | Baybayin: `ᜑᜊᜅ᜔`, `ᜁᜊᜎᜒᜃ᜔`)
  - **Comparisons & Assignment**: `is` (`=`), `isliterally` (`==`/`===`), `not` (`!=`)
- **Interactive User Input**: Built-in input function `pahingi` / `ask` / `ᜉᜑᜒᜅᜒ` for reading standard user input at runtime.
- **Type Conversion System**: Conversion built-in `isalin` / `convert` / `ᜁᜐᜎᜒᜈ᜔` supporting target datatypes:
  - Integer (`int`, `numero`, `bilang`, `ᜊᜒᜎᜅ᜔`, `ᜈᜓᜋᜒᜇᜓ`)
  - Float (`float`, `decimal`, `hatian`, `ᜑᜆᜒᜀᜈ᜔`)
  - String (`string`, `salita`, `ᜐᜎᜒᜆ`)
  - Boolean (`bool`, `boolean`, `booleano`, `tamao-mali`, `tamaomali`)
- **Logical Operators**: Support for logical operations in expressions:
  - Logical OR: `o`, `||`, `ᜂ`, `or`
  - Logical AND: `at`, `&&`, `ᜀᜆ᜔`, `and`
  - Logical NOT / Inequality: `hindi`, `!=`, `ᜑᜒᜈ᜔ᜇᜒ`, `not`
- **String Escape Characters**: Parsing and evaluation of escape sequences in string literals (`\n`, `\t`, `\r`, `\"`, `\\`).
- **Statement Terminator Extension**: Added standard semicolon `;` as a statement terminator alongside Single Danda (`᜵`) and Double Danda (`᜶`).
- **Categorized Multi-Lingual Examples**: Reorganized example scripts into language-specific folders:
  - `examples/tagalog/`
  - `examples/english/`
  - `examples/baybayin/`

## [0.1.1] - 2026-08-07

### Fixed
- **Baybayin Script Typos**: Fixed typos in Baybayin script tokens (`ᜁᜉᜎᜒᜏᜈᜄ᜔`, `ᜁᜉᜅᜎᜈ᜔`, `ᜑᜊᜅ᜔`) across lexer, tests, and documentation.
- **Type Conversion Datatype Matching**: Added Baybayin script representations (`ᜈᜓᜋᜒᜇᜓ` / `ᜈᜓᜋ᜔ᜁᜇᜓ`) to runtime type conversion target parsing.

---

## [0.1.0] - 2026-08-06

### Added
- **Rust Engine Core**: Fully replaced the legacy Python prototype (`Sawa`) with a performance-focused interpreter written in Rust (`Kalawang`).
- **Unicode Lexer**: Tokenizer with full support for UTF-8 and the Baybayin Unicode block (`U+1700` .. `U+171F`).
- **Dual Syntax & Keywords**: Native recognition for both Tagalog Latin keywords and authentic Baybayin script symbols:
  - **Output / Printing**: `sabihin`, `ipaliwanag`, `print`, `ᜐᜊᜒᜑᜒᜈ᜔`, `ᜁᜉᜎᜒᜏᜈᜄ᜔`
  - **Variable Declarations**: `si`, `ipangalan`, `var`, `ᜐᜒ`, `ᜁᜉᜅᜎᜈ᜔`
  - **Conditionals**: `kung` / `ᜃᜓᜅ᜔` (if), `okaya` / `ukaya` / `ᜂᜃᜌ` (else if), `kundi` / `ᜃᜓᜈ᜔ᜇᜒ` (else)
  - **Loops**: `habang` / `ᜑᜊᜅ᜔` (while loop)
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
