# Rust Core: Idiomatic Patterns & Philosophy

## Core Philosophy

1.  **Safety First**: `unsafe` is forbidden unless the user explicitly requests it and provides a rationale. Even then, you must wrap it in a `// SAFETY:` comment.
2.  **Expression-Oriented**: Rust is an expression language. Use this.
    - _Bad_: `let mut x = 0; if condition { x = 1; } else { x = 2; }`
    - _Good_: `let x = if condition { 1 } else { 2 };`
3.  **Type-Driven Design**: Make invalid states unrepresentable. Use `enum`s to encode state machines.

## Idiomatic Patterns

### Code organization

- Longer files are ok in Rust.
- Make sure functionality is encapsulated by a file/module
- Avoid deep nesting, prefer splitting up in function
    __bad__: 
    ```rust
    for tup in vector {
        for item in tup {
            match item {
                ...
            }
        }
    }
    ```
    __good__: 
    ```rust
    for tup in vector {
        operate_on_tup(tup)
    }

    fn operation_on_tup(tup: (&str, &str, &str)) {
        for item in tup {
            parse_item(item)
        }
    }

    fn parse_item(&str) -> Item {
        ...
    }
    ```
- Prefer usage of Rust traits, e.g. `From` for type conversions.

### Iterators vs Loops

- Prefer `Iterator` combinators for transformation and filtering.
- _Bad_:
  ```rust
  let mut results = Vec::new();
  for item in items {
      if item.is_valid() {
          results.push(item.process());
      }
  }
  ```
- _Good_:
  ```rust
  let results: Vec<_> = items.iter()
      .filter(|i| i.is_valid())
      .map(|i| i.process())
      .collect();
  ```

### Option & Result Combinators

- Use `map`, `and_then`, `unwrap_or_else`.
- Avoid excessive `if let Some(x) = y` nesting. - _Better_: `let value = y.ok_or(MyError::Missing)?.process();`

## Project Strictness

- **Async/Await**: Use `tokio` as the default runtime.
- **Formatting**: Strictly adhere to `rustfmt`. Code must pass `cargo fmt --check`.
- **Modules**: Keep `main.rs` small. Move logic to `lib.rs` or submodules (`src/my_module/mod.rs` or `src/my_module.rs`).
- **Visibility**: All fields in structs are private by default. Use `pub(crate)` for internal sharing, `pub` only for API surface.