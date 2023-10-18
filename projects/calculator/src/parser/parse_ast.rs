use super::*;
#[automatically_derived]
impl YggdrasilNode for ExpressionNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            add: pair.take_tagged_one::<AddNode>(Cow::Borrowed("add"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for ExpressionNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Expression)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AddNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            add_2: pair.take_tagged_option::<Add2Node>(Cow::Borrowed("add_2")),
            mul: pair.take_tagged_one::<MulNode>(Cow::Borrowed("mul"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for AddNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Add)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for Add2Node {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            add_2: pair.take_tagged_box::<Add2Node>(Cow::Borrowed("add_2"))?,
            mul: pair.take_tagged_one::<MulNode>(Cow::Borrowed("mul"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for Add2Node {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Add2)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MulNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            mul_2: pair.take_tagged_option::<Mul2Node>(Cow::Borrowed("mul_2")),
            pow: pair.take_tagged_one::<PowNode>(Cow::Borrowed("pow"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for MulNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Mul)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for Mul2Node {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            mul_2: pair.take_tagged_box::<Mul2Node>(Cow::Borrowed("mul_2"))?,
            pow: pair.take_tagged_one::<PowNode>(Cow::Borrowed("pow"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for Mul2Node {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Mul2)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for PowNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atom: pair.take_tagged_one::<AtomNode>(Cow::Borrowed("atom"))?,
            pow: pair.take_tagged_box::<PowNode>(Cow::Borrowed("pow"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for PowNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Pow)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for AtomNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            number: pair.take_tagged_one::<NumberNode>(Cow::Borrowed("number"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for AtomNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Atom)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for NumberNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for NumberNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Number)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for IntegerNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for IntegerNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Integer)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for WhiteSpaceNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self { span: Range { start: _span.start() as u32, end: _span.end() as u32 } })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::WhiteSpace)?)
    }
}
