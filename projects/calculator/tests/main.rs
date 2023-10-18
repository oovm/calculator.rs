use calculator::parser::{CalculatorParser, CalculatorRule, ExpressionNode};
use yggdrasil_rt::{YggdrasilNode, YggdrasilParser};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_ascii() {
    let cst = CalculatorParser::parse_cst("1 + 2 * 3 * 4 + 4 ^ 5 ^ 6", CalculatorRule::Expression).unwrap();
    println!("Short Form:\n{}", cst);
    // let first = ExpressionNode::from_cst(cst).unwrap();
    // println!("{:#?}", first)
}
#[test]
fn test_ascii2() {
    let cst = CalculatorParser::parse_cst("1 * 2 * 3", CalculatorRule::Mul).unwrap();
    println!("Short Form:\n{}", cst);
    // let first = ExpressionNode::from_cst(cst).unwrap();
    // println!("{:#?}", first)
}
