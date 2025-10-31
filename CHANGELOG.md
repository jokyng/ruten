# Changelog

All notable changes to **Ruten** will be documented in this file.  
This project follows a loose versioning model based on feature stability and module maturity.

---

## [1.1.2] — 31/10/2025
### Stable Release
- Marked as **stable** after multiple internal refactors and module consistency checks.  
- Improved interpreter performance and memory handling.  
- Unified all API integrations under the `apis/` namespace.  
- Added CI for Linux, macOS, and Windows (GitHub Actions).  
- Optimized `subproc` and `multiproc` modules for better concurrency.  
- Introduced new demos: `blockchain.rtn`, `news.rtn`, and `utils.rtn`.

---

## [1.1.0] — 21/10/2025
### Expanded Core Modules
- Added **AI**, **Vision**, and **NLP** modules for text and image tasks.  
- Introduced `torrent`, `downloads`, and `apps` modules.  
- Extended utilities with random generation, UUIDs, and environment variables.  
- Added `webhook` server module (local event handling).  
- Improved JSON parser speed (~1.5× faster).

---

## [1.0.0] — 16/10/2025
### First Public Build
- First stable CLI build for Windows, macOS, and Linux.  
- Added `blockchain`, `crypto_prices`, `weather`, and `news` modules.  
- Introduced `http` client with async support and basic headers.  
- Added syntax for functions, loops, conditionals, and dictionaries.  
- Began implementing module loader and sandboxed runtime.

---

## [0.6.x] — 12/10/2025
### Experimental Expansion
- Introduced multiple **experimental APIs**: `github`, `facts`, `quotes`, `geo`.  
- Added prototype of REPL with minimal command history.  
- Improved error messages and stack trace readability.  
- Began defining module architecture (`core/`, `apis/`, `utilities/`, `advanced/`).

---

## [0.4.x] — 10/9/2025
### Core Interpreter Rewrite
- Rewrote parser and evaluator in Rust for better performance.  
- Introduced scoped variables and improved function definitions.  
- Added experimental modules: `math`, `strings`, `json`, `crypto`.  
- Implemented REPL commands: `help`, `clear`, `exit`.

---

## [0.1.x] — 5/8/2025
### Initial Prototype
- Conceptual draft of the Ruten language.  
- Implemented lexer, parser, and primitive runtime in Rust.  
- Early experiments with modular imports.  
- Added first working REPL with `print()` and arithmetic expressions.  

---

made with <a href="https://github.com/jokyng/ruten"><code>jokyng</code></a> by <a href="https://github.com/ogcae"><code>ogcae</code></a>