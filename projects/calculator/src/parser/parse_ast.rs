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
            add: pair.take_tagged_box::<AddNode>(Cow::Borrowed("add"))?,
            rhs: pair.take_tagged_one::<MulNode>(Cow::Borrowed("rhs"))?,
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
impl YggdrasilNode for MulNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            mul: pair.take_tagged_box::<MulNode>(Cow::Borrowed("mul"))?,
            rhs: pair.take_tagged_one::<PowNode>(Cow::Borrowed("rhs"))?,
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
impl YggdrasilNode for PowNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            rhs: pair.take_tagged_box::<PowNode>(Cow::Borrowed("rhs"))?,
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
impl YggdrasilNode for AtomicNode {
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
impl FromStr for AtomicNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Atomic)?)
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
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
        Ok(Self {
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for WhiteSpaceNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::WhiteSpace)?)
    }
}
