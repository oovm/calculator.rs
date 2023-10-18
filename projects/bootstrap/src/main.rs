use std::path::Path;
use yggdrasil_shared::codegen::RustCodegen;

fn main() -> std::io::Result<()> {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));
    let input = include_str!("../grammars/calculator.ygg");
    let output = here.join("../calculator/src/parser").canonicalize()?;
    let builder = RustCodegen::default();
    builder.generate(input, output).unwrap();
    Ok(())
}
