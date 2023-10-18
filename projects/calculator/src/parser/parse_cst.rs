use super::*;

pub(super) fn parse_cst(input: &str, rule: CalculatorRule) -> OutputResult<CalculatorRule> {
    state(input, |state| match rule {
        CalculatorRule::Expression => parse_expression(state),
        CalculatorRule::Add => parse_add(state),
        CalculatorRule::Add2 => parse_add_2(state),
        CalculatorRule::Mul => parse_mul(state),
        CalculatorRule::Mul2 => parse_mul_2(state),
        CalculatorRule::Pow => parse_pow(state),
        CalculatorRule::Pow2 => parse_pow_2(state),
        CalculatorRule::Atom => parse_atom(state),
        CalculatorRule::OP_ADD => parse_op_add(state),
        CalculatorRule::OP_MUL => parse_op_mul(state),
        CalculatorRule::OP_POW => parse_op_pow(state),
        CalculatorRule::WhiteSpace => parse_white_space(state),
        CalculatorRule::IgnoreText => unreachable!(),
        CalculatorRule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_expression(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| parse_add(s).and_then(|s| s.tag_node("add")))
}
#[inline]
fn parse_add(state: Input) -> Output {
    state.rule(CalculatorRule::Add, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_mul(s).and_then(|s| s.tag_node("mul")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_add_2(s).and_then(|s| s.tag_node("add_2"))))
        })
    })
}
#[inline]
fn parse_add_2(state: Input) -> Output {
    state.rule(CalculatorRule::Add2, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_op_add(s).and_then(|s| s.tag_node("op_add")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_mul(s).and_then(|s| s.tag_node("mul")))
        })
    })
}
#[inline]
fn parse_mul(state: Input) -> Output {
    state.rule(CalculatorRule::Mul, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_pow(s).and_then(|s| s.tag_node("pow")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_mul_2(s).and_then(|s| s.tag_node("mul_2"))))
        })
    })
}
#[inline]
fn parse_mul_2(state: Input) -> Output {
    state.rule(CalculatorRule::Mul2, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_op_mul(s).and_then(|s| s.tag_node("op_mul")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_pow(s).and_then(|s| s.tag_node("pow")))
        })
    })
}
#[inline]
fn parse_pow(state: Input) -> Output {
    state.rule(CalculatorRule::Pow, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_atom(s).and_then(|s| s.tag_node("atom")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_pow_2(s).and_then(|s| s.tag_node("pow_2"))))
        })
    })
}

#[inline]
fn parse_pow_2(state: Input) -> Output {
    state.rule(CalculatorRule::Pow2, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_op_pow(s).and_then(|s| s.tag_node("op_pow")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_atom(s).and_then(|s| s.tag_node("atom")))
        })
    })
}
#[inline]
fn parse_atom(state: Input) -> Output {
    state.rule(CalculatorRule::Atom, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(0|[1-9][0-9]*)").unwrap())
        })
    })
}
#[inline]
fn parse_op_add(state: Input) -> Output {
    state.rule(CalculatorRule::OP_ADD, |s| s.match_string("+", false))
}
#[inline]
fn parse_op_mul(state: Input) -> Output {
    state.rule(CalculatorRule::OP_MUL, |s| s.match_string("*", false))
}
#[inline]
fn parse_op_pow(state: Input) -> Output {
    state.rule(CalculatorRule::OP_POW, |s| s.match_string("^", false))
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(CalculatorRule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([\\p{WhiteSpace}])").unwrap())
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s))
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
