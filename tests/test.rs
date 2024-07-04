use lerror::{bail, Context, ContextExt, Result};

#[test]
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
