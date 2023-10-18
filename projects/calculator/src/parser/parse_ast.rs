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
            additive: pair.take_tagged_one::<AdditiveNode>(Cow::Borrowed("additive"))?,
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
impl YggdrasilNode for AdditiveNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            multiplicative: pair.take_tagged_one::<MultiplicativeNode>(Cow::Borrowed("multiplicative"))?,
            rhs: pair.take_tagged_one::<AdditiveNode>(Cow::Borrowed("rhs"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for AdditiveNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Additive)?)
    }
}
#[automatically_derived]
impl YggdrasilNode for MultiplicativeNode {
    type Rule = CalculatorRule;

    fn get_range(&self) -> Option<Range<usize>> {
        Some(Range { start: self.span.start as usize, end: self.span.end as usize })
    }
    fn from_pair(pair: TokenPair<Self::Rule>) -> Result<Self, YggdrasilError<Self::Rule>> {
        let _span = pair.get_span();
        Ok(Self {
            atomic: pair.take_tagged_one::<AtomicNode>(Cow::Borrowed("atomic"))?,
            rhs: pair.take_tagged_one::<MultiplicativeNode>(Cow::Borrowed("rhs"))?,
            span: Range { start: _span.start() as u32, end: _span.end() as u32 },
        })
    }
}
#[automatically_derived]
impl FromStr for MultiplicativeNode {
    type Err = YggdrasilError<CalculatorRule>;

    fn from_str(input: &str) -> Result<Self, YggdrasilError<CalculatorRule>> {
        Self::from_cst(CalculatorParser::parse_cst(input, CalculatorRule::Multiplicative)?)
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
