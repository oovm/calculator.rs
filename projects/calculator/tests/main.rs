use calculator::parser::{CalculatorParser, CalculatorRule};
use yggdrasil_rt::YggdrasilParser;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_ascii() {
    let cst = CalculatorParser::parse_cst("1 + 2 + 3 * 4 * 5 + 6 ^ 7 ^ 8", CalculatorRule::Expression).unwrap();
    println!("Short Form:\n{}", cst);
    // let first = ExpressionNode::from_cst(cst).unwrap();
    // println!("{:#?}", first)
}

#[test]
fn test_ascii2() {
    let cst = CalculatorParser::parse_cst("1 ^ 2 ^ 3", CalculatorRule::Pow).unwrap();
    println!("Short Form:\n{}", cst);
    // let first = ExpressionNode::from_cst(cst).unwrap();
    // println!("{:#?}", first)
}
