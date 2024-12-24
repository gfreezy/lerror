use lerror::{bail, Context, ContextExt, Result};

fn a() -> Result<()> {
    b().c()?;
    bail!("permission denied for accessing {}", "resource");
}

fn b() -> Result<()> {
    c().context("File not found")?;
    bail!("File not found");
}

fn c() -> Result<()> {
    bail!("Image not found");
}

#[test]
fn test_io_error() -> Result<()> {
    let c = c().unwrap_err();
    assert_eq!(
        format!("{:?}", c),
        "lerror::Error\n\n    0: tests/test.rs:15:5\n       Image not found\n"
    );
    let b = b().unwrap_err();
    assert_eq!(
        format!("{:?}", b),
        "lerror::Error\n\n    0: tests/test.rs:10:9\n       File not found\n    1: tests/test.rs:15:5\n       Image not found\n"
    );
    Ok(())
}
