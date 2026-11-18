# Learning Rust

A hands-on Rust course for someone coming from JavaScript/TypeScript.

## How it works

Each topic has two parts:

| Folder                | What's in it                                             | How to use it                                                    |
| --------------------- | -------------------------------------------------------- | ---------------------------------------------------------------- |
| `lessons/NN-topic/`   | `README.md` with the explanation + runnable example code | Read the README, then `cargo run -p lesson-NN`                   |
| `exercises/NN-topic/` | Functions with `todo!()` and tests that fail             | Replace each `todo!()` until `cargo test -p exercises-NN` passes |

`todo!()` is a built-in macro that compiles but panics when it runs, so every
test starts out red. Your job is to make them green.

## Commands

```bash
cargo run -p lesson-01          # run the lesson example
cargo test -p exercises-01      # run the exercise tests
cargo test -p exercises-01 sum  # run only tests whose name contains "sum"
cargo clippy -p exercises-01    # extra lint hints, often teaches idiomatic Rust
cargo fmt                       # format all code
```

## Path

1. **Basics** – Cargo, variables, types, functions, control flow ← _you are here_
2. Ownership & borrowing
3. Structs, enums & `match`
4. Error handling: `Option`, `Result`, `?`
5. Collections, iterators & closures
6. Traits & generics
7. Modules, crates & testing
8. Project: Azure cost export CLI
