// Lesson 1 – Basics. Run with: cargo run -p lesson-01

fn main() {
    variables();
    types();
    println!("add(2, 3) = {}", add(2, 3));
    control_flow();
}

fn variables() {
    println!("--- variables ---");

    let x = 5; // immutable, like `const`
    let mut y = 5; // mutable, like `let`
    y += 1;
    println!("x = {x}, y = {y}");

    // Shadowing: a new variable with the same name, here even a new type.
    let input = "42";
    let input: i32 = input.parse().unwrap();
    println!("input + 1 = {}", input + 1);
}

fn types() {
    println!("--- types ---");

    let count: u32 = 3;
    let price: f64 = 9.99;
    // No implicit conversion: `count * price` would not compile.
    let total = count as f64 * price;
    println!("{count} items × {price} = {total:.2}");

    let letter: char = 'R';
    let greeting: &str = "hello";
    let owned: String = format!("{greeting}, {letter}ust!");
    println!("{owned}");
}

// The last expression (no semicolon) is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn control_flow() {
    println!("--- control flow ---");

    // `if` is an expression, so it replaces the ternary operator.
    let n = -4;
    let sign = if n > 0 { "positive" } else { "not positive" };
    println!("{n} is {sign}");

    // `for` over a range: 1..=5 includes 5.
    let mut sum = 0;
    for i in 1..=5 {
        sum += i;
    }
    println!("sum of 1..=5 = {sum}");

    // `loop` can return a value with `break`.
    let mut attempts = 0;
    let found = loop {
        attempts += 1;
        if attempts * attempts > 50 {
            break attempts;
        }
    };
    println!("first number whose square > 50: {found}");
}
