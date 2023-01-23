# Lamb

`lamb` is an implementation of the pure untyped lambda calculus in modern, safe Rust.

# Installation

- ## Library

  Add `lamb` as a dependency in `Cargo.toml`:
  ```toml
  [dependencies]
  lamb = "0.1.0"
  ```

  The Cargo features `repl` and `prelude` can also be enabled to interface with the REPL and prelude:
  ```toml
  [dependencies]
  lamb = { version = "0.1.0", features = ["repl", "prelude"] }
  ```

- ## Binary

  Install `lamb` through Cargo:
  ```
  cargo install lamb
  ```

# Features

- ## Library

  Default:
  - Construct terms programmatically
  - β-reduce terms using different reduction strategies
  - Implement custom β-reduction strategies

  With `prelude` enabled:
  - Use pre-defined terms from the prelude

  With `repl` enabled:
  - Parse terms from strings
  - Construct REPLs programmatically and execute commands

- ## Binary

  - β-reduce terms using any pre-defined β-reduction strategy:
    ```
    λ> (λx. x) (w z)
    Info: Reduced 1 times
    w z
    λ>
    ```
  - Bind terms to names to automatically substitute in future free variables:
    ```
    λ> id = λx. x; const = λx y. y;
    Info: Binding id added
    Info: Binding const added
    λ>
    ```
  - Display or change the β-reduction limit:
    ```
    λ> :limit 1024
    Info: Reduction limit set to 1024
    λ> :limit
    Info: Current reduction limit is 1024
    λ>
    ```
  - Exit gracefully:
    ```
    λ> :exit
    ```