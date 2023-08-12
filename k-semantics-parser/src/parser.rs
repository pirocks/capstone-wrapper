use crate::tokenize::Token;

#[derive(Debug)]
pub enum FunctionToCall {
    UpdateMap,
    ConvToRegKeys,
    ExtractMInt,
    AddMInt,
    AndMInt,
    EqMInt,
    GetFlag,
    Mi,
    ConcatenateMInt,
    GetParentValue,
    NotBool,
    XorBool,
    XorMInt,
    AndBool,
    HandleImmediateWithSignExtend,
    AddDouble,
    LshrMInt,
    UvalueMInt,
    NegMInt,
    ShiftLeftMInt
}

impl FunctionToCall {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        let s = s.as_str();
        match s {
            "updateMap" => Self::UpdateMap,
            "convToRegKeys" => Self::ConvToRegKeys,
            "extractMInt" => Self::ExtractMInt,
            "addMInt" => Self::AddMInt,
            "andMInt" => Self::AndMInt,
            "eqMInt" => Self::EqMInt,
            "getFlag" => Self::GetFlag,
            "concatenateMInt" => Self::ConcatenateMInt,
            "mi" => Self::Mi,
            "getParentValue" => Self::GetParentValue,
            "notBool" => Self::NotBool,
            "xorBool" => Self::XorBool,
            "xorMInt" => Self::XorMInt,
            "andBool" => Self::AndBool,
            "handleImmediateWithSignExtend" => Self::HandleImmediateWithSignExtend,
            "add_double" => Self::AddDouble,
            "lshrMInt" => Self::LshrMInt,
            "uvalueMInt" => Self::UvalueMInt,
            "negMInt" => Self::NegMInt,
            "shiftLeftMInt" => Self::ShiftLeftMInt,
            _ => panic!("{}", s)
        }
    }
}

#[derive(Debug)]
pub enum ConstantParameter {
    RSMap,
    R1,
    R2,
    R3,
    R4,
    CF,
    PF,
    AF,
    ZF,
    SF,
    OF,
    RIP,
    Imm32,
    Imm8,
    UndefMInt,
    Mem64
}

impl ConstantParameter {
    pub fn new(s: impl Into<String>) -> Self {
        let s = s.into();
        match s.as_str() {
            "RSMap" => Self::RSMap,
            "R1" => Self::R1,
            "R2" => Self::R2,
            "R3" => Self::R3,
            "R4" => Self::R4,
            "CF" => Self::CF,
            "PF" => Self::PF,
            "AF" => Self::AF,
            "ZF" => Self::ZF,
            "SF" => Self::SF,
            "OF" => Self::OF,
            "RIP" => Self::RIP,
            "Imm32" => Self::Imm32,
            "Imm8" => Self::Imm8,
            "undefMInt" => Self::UndefMInt,
            "Mem64" => Self::Mem64,
            _ => todo!("{}", s.as_str())
        }
    }
}

#[derive(Debug)]
pub enum KSemanticsExpr {
    FunctionCall {
        name: FunctionToCall,
        args: Vec<KSemanticsExpr>,
    },
    Equals {
        left: Box<KSemanticsExpr>,
        right: Box<KSemanticsExpr>,
    },
    IfElse {
        condition: Box<KSemanticsExpr>,
        true_case: Box<KSemanticsExpr>,
        false_case: Box<KSemanticsExpr>,
    },
    ConstParam(ConstantParameter),
    Num(u64),
    ImplicationSequence(Vec<(KSemanticsExpr, KSemanticsExpr)>),
    RIPLookup {},
    CoerceMInt {
        inner: Box<KSemanticsExpr>
    },
}

#[derive(Debug)]
pub enum KSemanticsAST {
    Expr(KSemanticsExpr),
}

#[derive(Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
}

impl Tokens {
    pub fn new(mut tokens: Vec<Token>) -> Self {
        tokens.reverse();
        Self {
            tokens,
        }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop().unwrap()
    }

    pub fn nth(&self, i: usize) -> Option<&Token> {
        if self.tokens.len() < i + 1 {
            return None;
        }
        Some(&self.tokens[self.tokens.len() - i - 1])
    }

    pub fn expect(&mut self, token: Token) {
        assert_eq!(self.next(), token);
    }

    pub fn parse_function_args(&mut self, function_to_call: FunctionToCall) -> KSemanticsExpr {
        let mut args = vec![];
        self.expect(Token::OpenParen);
        loop {
            args.push(self.parse_expression());
            if let Some(Token::Comma) = self.nth(0) {
                self.expect(Token::Comma)
            } else {
                break;
            }
        }
        self.expect(Token::CloseParen);
        KSemanticsExpr::FunctionCall { name: function_to_call, args }
    }

    pub fn parse_expression(&mut self) -> KSemanticsExpr {
        let mut expr_array = vec![];
        loop {
            let next_token = self.next();
            let res = match next_token {
                Token::Ident(ident) => {
                    if let Some(Token::OpenParen) = self.nth(0) {
                        let function_to_call = FunctionToCall::new(ident);
                        self.parse_function_args(function_to_call)
                    } else if let Some(Token::Comma) | Some(Token::CloseParen) = self.nth(0) {
                        KSemanticsExpr::ConstParam(ConstantParameter::new(ident))
                    } else if let Some(Token::OpenSquareParen) = self.nth(0) {
                        if ident.as_str() == "RSMap" {
                            self.expect(Token::OpenSquareParen);
                            self.expect(Token::Str("RIP".to_string()));
                            self.expect(Token::CloseSquareParen);
                            KSemanticsExpr::RIPLookup {}
                        } else {
                            todo!()
                        }
                    } else {
                        dbg!(self.nth(0));
                        todo!()
                    }
                }
                Token::Str(str) => {
                    KSemanticsExpr::ConstParam(ConstantParameter::new(str))
                }
                Token::Number(num) => {
                    KSemanticsExpr::Num(num)
                }
                Token::Implication => {
                    dbg!(&self.tokens);
                    todo!()
                }
                Token::OpenParen => {
                    let inner = self.parse_expression();
                    self.expect(Token::CloseParen);
                    inner
                }
                Token::OpenSquareParen => {
                    todo!()
                }
                Token::OpenCurlyParen => {
                    let inner = self.parse_expression();
                    self.expect(Token::CloseCurlyParen);
                    self.expect(Token::ColonGreater);
                    self.expect(Token::Ident("MInt".to_string()));
                    KSemanticsExpr::CoerceMInt { inner: Box::new(inner) }
                }
                Token::CloseParen => {
                    todo!()
                }
                Token::CloseSquareParen => {
                    todo!()
                }
                Token::CloseCurlyParen => {
                    todo!()
                }
                Token::Comma => {
                    todo!()
                }
                Token::If => {
                    let condition = self.parse_expression();
                    self.expect(Token::Then);
                    let true_case = self.parse_expression();
                    self.expect(Token::Else);
                    let false_case = self.parse_expression();
                    self.expect(Token::Fi);
                    KSemanticsExpr::IfElse {
                        condition: Box::new(condition),
                        true_case: Box::new(true_case),
                        false_case: Box::new(false_case),
                    }
                }
                Token::Then => {
                    todo!()
                }
                Token::Else => {
                    todo!()
                }
                Token::Fi => {
                    todo!()
                }
                Token::Whitespace => {
                    panic!("Whitespace should have been removed")
                }
                Token::Equal => {
                    todo!()
                }
                Token::ColonGreater => {
                    todo!()
                }
            };
            if let Some(Token::Implication) = self.nth(0) {
                self.expect(Token::Implication);
                let next_expression = self.parse_expression();
                expr_array.push((res, next_expression));
                if let Some(Token::CloseParen) = self.nth(0) {
                    return KSemanticsExpr::ImplicationSequence(expr_array);
                }
                continue;
            }
            if let Some(Token::Ident(next_ident)) = self.nth(0) {
                let next_ident = next_ident.as_str();
                if next_ident == "xorBool" || next_ident == "xorMInt" || next_ident == "andBool" {
                    let name = FunctionToCall::new(next_ident);
                    let _ = self.next();
                    let next_expression = self.parse_expression();
                    return KSemanticsExpr::FunctionCall { name, args: vec![res, next_expression] };
                }
            }
            if let Some(Token::Equal) = self.nth(0) {
                self.expect(Token::Equal);
                let next_expression = self.parse_expression();
                return KSemanticsExpr::Equals {
                    left: Box::new(res),
                    right: Box::new(next_expression),
                };
            }
            return res;
        }
    }
}
