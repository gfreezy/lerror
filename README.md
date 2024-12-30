# lerror
Another rust error crate.

Report file name, line and column instead of backtrace.

Most code is copied from `anyhow`.

# Usage
```rust
use lerror::{bail, Context, ContextExt, Result};

#[test]
fn a() -> Result<()> {
    b().context("custom error message")?;  // You need to call `context()` to add the current line to backtrace with string context.
    bail!("permission denied for accessing {}", "resource");
    c().with_context(|| "custom error message")?;  // You can also call `with_context()` to add the current line to backtrace with string context.
    Ok(())
}

fn b() -> Result<()> {
    c().context("File not found")?;
    bail!("File not found");
}

fn c() -> Result<()> {
    bail!("Image not found");
}
```

Output:

```
Error: lerror::Error

    0: tests/test.rs:5:9
    1: tests/test.rs:11:9
       File not found
    2: tests/test.rs:16:5
       Image not found
```

# lerror-macros

This crate provides a macro for automatically adding context to all `?` operators in a function. This eliminates the need to manually call `.c()` or `.context()`.

```rust
use lerror_macros::lerror_trace;
use lerror::{Result, Context};

#[lerror_trace]
fn a() -> Result<()> {
    b()?;
    Ok(())
}
```