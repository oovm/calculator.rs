use super::*;

pub(super) fn parse_cst(input: &str, rule: CalculatorRule) -> OutputResult<CalculatorRule> {
    state(input, |state| match rule {
        CalculatorRule::Expression => parse_expression(state),
        CalculatorRule::Add => parse_expression_add(state),
        CalculatorRule::Add2 => parse_expression_add_residual(state),
        CalculatorRule::Mul => parse_expression_mul(state),
        CalculatorRule::Mul2 => parse_expression_mul_residual(state),
        CalculatorRule::Pow => parse_pow(state),
        CalculatorRule::Pow2 => parse_expression_pow_residual(state),
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
    state.rule(CalculatorRule::Expression, |s| {
        parse_expression_add(s).and_then(|s| s.tag_node("add")).and_then(crate::helpers::climb)
    })
}

#[inline]
fn parse_expression_add(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s) //
                .and_then(|s| parse_expression_mul(s).and_then(|s| s.tag_node("mul")))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s) //
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_expression_add_residual(s).and_then(|s| s.tag_node("add")))
                        })
                    })
                })
        })
    })
}

#[inline]
fn parse_expression_add_residual(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_op_add(s).and_then(|s| s.tag_node("op_add")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_expression_mul(s).and_then(|s| s.tag_node("mul")))
        })
    })
}

#[inline]
fn parse_expression_mul(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s) //
                .and_then(|s| parse_pow(s).and_then(|s| s.tag_node("pow")))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s) //
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_expression_mul_residual(s).and_then(|s| s.tag_node("mul")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_expression_mul_residual(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| {
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
    state.rule(CalculatorRule::Expression, |s| {
        s.sequence(|s| {
            Ok(s) //
                .and_then(|s| parse_atom(s).and_then(|s| s.tag_node("atom")))
                .and_then(|s| {
                    s.repeat(0..4294967295, |s| {
                        s.sequence(|s| {
                            Ok(s) //
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_expression_pow_residual(s).and_then(|s| s.tag_node("pow")))
                        })
                    })
                })
        })
    })
}
#[inline]
fn parse_expression_pow_residual(state: Input) -> Output {
    state.rule(CalculatorRule::Expression, |s| {
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
