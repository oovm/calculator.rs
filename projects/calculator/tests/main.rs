
use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};
use calculator::parser::{CalculatorParser, CalculatorRule, ExpressionNode};

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test_ascii() {
    let cst = CalculatorParser::parse_cst("1 + 2 * 3 * 4 + 4 ^ 5 ^6", CalculatorRule::Expression).unwrap();
    println!("Short Form:\n{}", cst);
    let first = ExpressionNode::from_cst(cst).unwrap();
    println!("{:#?}", first)
}

#[test]
fn codegen() {
    let grammars = std::path::Path::new("grammars/").canonicalize().unwrap();
    let builder = yggdrasil_shared::codegen::RustCodegen::default();
    builder.generate(include_str!("grammars/calculator.ygg"), "src/parser").unwrap();
    println!("cargo:rerun-if-changed={}", grammars.display());
}
