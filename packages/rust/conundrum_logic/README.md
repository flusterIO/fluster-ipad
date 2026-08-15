
## Rhai - embedded scripting for Rust

![Rhai logo](https://rhai.rs/book/images/logo/rhai-banner-transparent-colour.svg)

Rhai is a tiny, simple and fast embedded scripting language for Rust
that gives you a safe and easy way to add scripting to your applications.

It provides a familiar syntax based on JavaScript+Rust and a simple Rust interface.

## A Quick Example

### Contents of `my_script.rhai`

```rhai
/// Brute force factorial function
fn factorial(x) {
    if x == 1 { return 1; }
    x * factorial(x - 1)
}

// Calling an external function 'compute'
compute(factorial(10))
```

### The Rust part

```rust
use rhai::{Engine, EvalAltResult};

fn main() -> Result<(), Box<EvalAltResult>>
{
    // Define external function
    fn compute_something(x: i64) -> bool {
        (x % 40) == 0
    }

    // Create scripting engine
    let mut engine = Engine::new();

    // Register external function as 'compute'
    engine.register_fn("compute", compute_something);

#
    // Evaluate the script, expecting a 'bool' result
    let result: bool = engine.eval_file("my_script.rhai".into())?;

    assert_eq!(result, true);

    Ok(())
}
```

## Features


Current version: 1.24.0

License: [Open-Source, but with a purpose](https://flusterapp.com/license)
