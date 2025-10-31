# contributing to ruten

thank you for your interest in contributing to ruten! we welcome contributions from the community.

---

## development setup

### prerequisites

- rust 1.70 or higher
- cargo (comes with rust)
- git

### getting started

```bash
# clone the repository
git clone https://github.com/ogcae/ruten
cd ruten

# build the project
cargo build

# run tests
cargo test

# install locally
cargo install --path .
```

---

## code style

### rust conventions

- use lowercase comments throughout the codebase
- follow standard rust naming conventions (snake_case for functions/variables)
- keep modules focused and single-purpose
- add comprehensive error handling
- write clear, descriptive variable names

### example

```rust
// good - lowercase comment, clear naming
pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    
    // add function to module
    module.insert("function_name".to_string(), value);
    
    module
}
```

---

## adding new modules

to add a new built-in module:

### 1. create module file

create `src/modules/your_module.rs`:

```rust
// your_module - description of what it does
// made by ogcae

use crate::error::RutenError;
use crate::interpreter::Value;
use std::collections::HashMap;

pub fn create_module() -> HashMap<String, Value> {
    let mut module = HashMap::new();
    
    // add your functions here
    module.insert(
        "function_name".to_string(),
        Value::NativeFunction(|args| {
            // implementation
            Ok(Value::None)
        }),
    );
    
    module
}
```

### 2. register module

add to `src/modules/mod.rs`:

```rust
pub mod your_module;

// in load_module function:
"your_module" => your_module::create_module(),
```

### 3. add documentation

update `README.md` and `DOCUMENTATION.md` with:
- module description
- function signatures
- usage examples

### 4. create examples

add `examples/your_module_demo.rtn`:

```ruten
# your_module demonstration
# made by ogcae

import your_module

# demonstrate functionality
result = your_module.function_name()
print(result)
```

---

## testing

### running tests

```bash
# run all tests
cargo test

# run specific test
cargo test test_name

# run with output
cargo test -- --nocapture
```

### testing examples

```bash
# run example script
cargo run -- examples/math_demo.rtn

# test repl
cargo run
```

### adding tests

add tests to your module file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        // test implementation
        assert_eq!(result, expected);
    }
}
```

---

## pull request process

### 1. fork and branch

```bash
# fork the repository on github
git clone https://github.com/ogcae/ruten
cd ruten

# create feature branch
git checkout -b feature/amazing-feature
```

### 2. make changes

- write clean, documented code
- follow existing code style
- add tests for new features
- update documentation

### 3. test thoroughly

```bash
# ensure all tests pass
cargo test

# check formatting
cargo fmt --check

# run clippy for lints
cargo clippy
```

### 4. commit and push

```bash
# commit with clear message
git commit -m "add amazing feature"

# push to your fork
git push origin feature/amazing-feature
```

### 5. create pull request

- go to github and create a pull request
- describe your changes clearly
- reference any related issues
- wait for review

---

## commit message guidelines

use clear, descriptive commit messages:

```
add webhook module for http server support
fix: resolve memory leak in interpreter
docs: update readme with new examples
refactor: simplify error handling in parser
test: add comprehensive crypto module tests
```

---

## reporting bugs

### before reporting

- check existing issues
- verify it's reproducible
- test with latest version

### bug report template

```markdown
**description**
clear description of the bug

**steps to reproduce**
1. step one
2. step two
3. step three

**expected behavior**
what should happen

**actual behavior**
what actually happens

**environment**
- ruten version: 2.0.0
- rust version: 1.70
- os: linux/macos/windows
```

---

## feature requests

we welcome feature requests! please include:

- clear description of the feature
- use cases and examples
- why it would benefit ruten users
- any implementation ideas

---

## code of conduct

### our standards

- be respectful and inclusive
- welcome newcomers
- focus on constructive feedback
- help others learn and grow

### unacceptable behavior

- harassment or discrimination
- trolling or insulting comments
- personal attacks
- publishing private information

---

## questions?

- open an issue for questions
- check existing documentation
- join discussions on github

---

## license

by contributing, you agree that your contributions will be licensed under the mit license.

---

made with <a href="https://github.com/jokyng/ruten"><code>jokyng</code></a> by <a href="https://github.com/ogcae"><code>ogcae</code>
</a>

*thank you for contributing to ruten!*
