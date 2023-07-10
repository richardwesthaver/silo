//! print-ast.rs --- print the default representation of ast for INPUT
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use quote::quote;
pub fn write_and_fmt<P: AsRef<Path>, S: ToString>(path: P, code: S) -> io::Result<()> {
    fs::write(&path, code.to_string())?;

    Command::new("rustfmt")
        .arg(path.as_ref())
        .spawn()?
        .wait()?;

    Ok(())
}

fn main() -> io::Result<()> {
  let mut args = std::env::args();
  if let Some(input) = args.nth(1) {
    let syntax = syn::parse_file(&input).unwrap();
    println!("AST:");
    println!("{}", quote!(#syntax));
  }
  Ok(())
}
