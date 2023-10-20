use crate::parser::CalculatorRule;
use yggdrasil_rt::State;

type Input<'i> = Box<State<'i, CalculatorRule>>;
type Output<'i> = Result<Box<State<'i, CalculatorRule>>, Box<State<'i, CalculatorRule>>>;

pub fn climb(state: Input) -> Output {
    Ok(state)
}
