# Lesson 1 – Basics

Run the example alongside this text: `cargo run -p lesson-01`
(code in [src/main.rs](src/main.rs)).

## 1. Cargo is npm + tsc + jest in one

| JS / Node           | Rust                                                  |
| ------------------- | ----------------------------------------------------- |
| `package.json`      | `Cargo.toml`                                          |
| `package-lock.json` | `Cargo.lock`                                          |
| `node_modules/`     | `target/` (compiled output) + a global download cache |
| npmjs.com           | crates.io (packages are called _crates_)              |
| `npm install x`     | `cargo add x`                                         |
| `node index.js`     | `cargo run` (compile + run)                           |
| `npm test`          | `cargo test` (tests are built in, no jest needed)     |

Rust is **compiled**: `cargo build` turns your code into a native executable.
Most bugs you'd find at runtime in JS are compile errors in Rust.

## 2. Variables are immutable by default

```rust
let x = 5;        // like `const` in JS
x = 6;            // ❌ compile error: cannot assign twice to immutable variable

let mut y = 5;    // like `let` in JS
y = 6;            // ✅
```

Default-immutable is the opposite of JS habits. Only add `mut` when you need it.

### Shadowing

You can declare a _new_ variable with the same name. That's not mutation, and
the type may even change:

```rust
let input = "42";              // &str (text)
let input: i32 = input.parse().unwrap();   // now an i32 (number)
```

In JS you'd need two names (`inputStr`, `inputNum`).

## 3. Types

Rust is statically typed like TypeScript, but types are **real**: they decide
memory layout and are checked strictly. Usually the compiler infers them.

| Type                  | Meaning                                         | JS equivalent |
| --------------------- | ----------------------------------------------- | ------------- |
| `i32`, `i64`          | signed integers (32/64-bit)                     | `number`      |
| `u32`, `u64`, `usize` | unsigned (≥ 0) integers; `usize` = index/length | `number`      |
| `f64`                 | floating point                                  | `number`      |
| `bool`                | `true` / `false`                                | `boolean`     |
| `char`                | one Unicode character, `'a'` (single quotes!)   | —             |
| `&str`                | a borrowed piece of text, `"hello"`             | `string`      |
| `String`              | owned, growable text                            | `string`      |

Key differences from JS:

- **No implicit conversions.** `5 + 2.0` is a compile error. Convert explicitly:
  `5 as f64 + 2.0`.
- **No `null` or `undefined`.** A value always exists. ("Maybe a value" is
  `Option`, lesson 4.)
- **Integer overflow is checked** in debug builds: it panics instead of
  silently becoming a float.
- `&str` vs `String` will make sense after lesson 2 (ownership). For now:
  string literals are `&str`, and `.to_string()` / `format!()` give a `String`.

## 4. Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b       // no semicolon = this is the return value
}
```

- Parameter types and the return type are **required** (no inference here).
- `-> i32` is the return type. No arrow = returns nothing (`()`, like `void`).

### Expressions vs statements: the semicolon matters

Almost everything in Rust is an _expression_ that produces a value. The last
expression in a block, **without** a semicolon, is the block's value.

```rust
fn double(n: i32) -> i32 {
    n * 2       // ✅ returns n * 2
}

fn double_broken(n: i32) -> i32 {
    n * 2;      // ❌ the semicolon turns it into a statement → returns ()
}
```

`return x;` exists too, but is mostly used for _early_ returns.

## 5. Control flow

### `if` is an expression

```rust
let label = if n > 0 { "positive" } else { "not positive" };
```

That replaces JS's ternary `n > 0 ? "positive" : "not positive"`.
The condition must be a real `bool`: `if n { }` with a number is a compile
error (no truthy/falsy).

### Loops

```rust
for i in 1..=5 { }      // 1, 2, 3, 4, 5   (..= includes the end)
for i in 0..5 { }       // 0, 1, 2, 3, 4   (.. excludes the end)

while count < 10 { count += 1; }

let result = loop {     // infinite loop; `break value` returns a value
    if done { break 42; }
};
```

There is no C-style `for (let i = 0; i < n; i++)`; ranges replace it.

## 6. Printing

```rust
let name = "Mike";
println!("Hello, {name}!");           // like `Hello, ${name}!`
println!("{} + {} = {}", 1, 2, 1 + 2);
println!("{:?}", (1, "two"));         // {:?} = debug format, like console.log
let s = format!("{name}!");           // like a template literal → String
```

The `!` means `println!` is a **macro** (code that generates code), not a
function. You'll mostly _use_ macros, rarely write them.

## Check yourself

Before running it, predict: does this compile? If not, why?

```rust
fn main() {
    let total = 0;
    for i in 1..=3 {
        total += i;
    }
    println!("{total}");
}
```

<details><summary>Answer</summary>

No: `total` isn't `mut`. Change it to `let mut total = 0;`. The compiler
tells you exactly this, and even suggests the fix. Read Rust errors fully,
they're usually that helpful.
</details>

## Exercises

Open [exercises/01-basics/src/lib.rs](../../exercises/01-basics/src/lib.rs)
and replace each `todo!()`. Check your work with:

```bash
cargo test -p exercises-01
```
