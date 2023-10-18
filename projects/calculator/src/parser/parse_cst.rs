use super::*;

pub(super) fn parse_cst(input: &str, rule: CalculatorRule) -> OutputResult<CalculatorRule> {
    state(input, |state| match rule {
        CalculatorRule::Expression => parse_expression(state),
        CalculatorRule::Add => parse_add(state),
        CalculatorRule::Mul => parse_mul(state),
        CalculatorRule::Pow => parse_pow(state),
        CalculatorRule::Atomic => parse_atomic(state),
        CalculatorRule::Number => parse_number(state),
        CalculatorRule::Integer => parse_integer(state),
        CalculatorRule::WhiteSpace => parse_white_space(state),
        CalculatorRule::IgnoreText => unreachable!(),
        CalculatorRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| {
        parse_add(s).and_then(|s| s.tag_node("add"))
    })
}
#[inline]
fn parse_add(state: Input) -> Output {
    state.rule(CalculatorRule::Add, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_add(s).and_then(|s| s.tag_node("add"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|s.sequence(|s|Ok(s).and_then(|s|builtin_text(s, "+", false)).and_then(|s|builtin_ignore(s)).and_then(|s|parse_mul(s).and_then(|s| s.tag_node("rhs"))))))).and_then(|s| s.tag_node("lhs"))
    })
}
#[inline]
fn parse_mul(state: Input) -> Output {
    state.rule(CalculatorRule::Mul, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_mul(s).and_then(|s| s.tag_node("mul"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|s.sequence(|s|Ok(s).and_then(|s|builtin_text(s, "*", false)).and_then(|s|builtin_ignore(s)).and_then(|s|parse_pow(s).and_then(|s| s.tag_node("rhs"))))))).and_then(|s| s.tag_node("lhs"))
    })
}
#[inline]
fn parse_pow(state: Input) -> Output {
    state.rule(CalculatorRule::Pow, |s| {
        s.sequence(|s|Ok(s).and_then(|s|s.optional(|s|s.sequence(|s|Ok(s).and_then(|s|parse_atomic(s).and_then(|s| s.tag_node("atomic"))).and_then(|s|builtin_ignore(s)).and_then(|s|builtin_text(s, "^", false))).and_then(|s| s.tag_node("lhs")))).and_then(|s|builtin_ignore(s)).and_then(|s|parse_pow(s).and_then(|s| s.tag_node("rhs"))))
    })
}
#[inline]
fn parse_atomic(state: Input) -> Output {
    state.rule(CalculatorRule::Atomic, |s| {
        parse_number(s).and_then(|s| s.tag_node("number"))
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(CalculatorRule::Number, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new("^([+-]?(0|[1-9][0-9]*))").unwrap())})
    })
}
#[inline]
fn parse_integer(state: Input) -> Output {
    state.rule(CalculatorRule::Integer, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new("^(0|[1-9][0-9]*)").unwrap())})
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(CalculatorRule::WhiteSpace, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(|| Regex::new("^([\\p{WhiteSpace}])").unwrap())})
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| {
        parse_white_space(s)
    })

}

fn builtin_any(state: Input) -> Output {
    state.rule(CalculatorRule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(CalculatorRule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(CalculatorRule::IgnoreRegex, |s| s.match_regex(regex))
}