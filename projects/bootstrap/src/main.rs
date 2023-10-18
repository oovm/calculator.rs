

fn main() -> std::io::Result<()> {
    let here = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    println!("{}")
    // let builder = yggdrasil_shared::codegen::RustCodegen::default();
    // builder.generate(include_str!("grammars/calculator.ygg"), "src/parser").unwrap();
    Ok(())
}
