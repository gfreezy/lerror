# lerror
Another rust error crate.

Report file name, line and column instead of backtrace.

Most code is copied from `anyhow`.

# Usage
```rust
use lerror::{bail, Context, ContextExt, Result};

#[test]
fn a() -> Result<()> {
    b().c()?;  // You need to call `c()` to add the current line to backtrace without context. Or you can call `context()` to add string context.
    bail!("permission denied for accessing {}", "resource");
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

#[lerror_trace]
fn a() -> Result<()> {
    b()?;
    Ok(())
}
```