use lerror::{bail, Context, Result};
use lerror_macros::lerror_trace;

#[lerror_trace]
fn a() -> Result<()> {
    b()?;
    bail!("permission denied for accessing {}", "resource");
}

#[lerror_trace]
fn b() -> Result<()> {
    c().context("file not found")?;
    bail!("File not found");
}

#[lerror_trace]
fn c() -> Result<()> {
    bail!("Image not found");
}

#[test]
fn test_io_error() -> Result<()> {
    let c = c().unwrap_err();
    assert_eq!(
        format!("{:?}", c),
        "lerror::Error\n\n    0: lerror/tests/test.rs:18:5\n       Image not found\n"
    );
    let b = b().unwrap_err();
    assert_eq!(
        format!("{:?}", b),
        "lerror::Error\n\n    0: lerror/tests/test.rs:12:9\n       file not found\n    1: lerror/tests/test.rs:18:5\n       Image not found\n"
    );
    Ok(())
}
