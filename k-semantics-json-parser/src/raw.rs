#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct OperandIdx(pub(crate) u8);


#[derive(Debug)]
pub enum RawExpression {
    Op(OperandIdx),
    IfElse {
        condition: Box<RawExpression>,
        true_case: Box<RawExpression>,
        false_case: Box<RawExpression>,
    },
    AndBool {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    EqualsBool {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    Equals {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    MI {
        len: Box<RawExpression>,
        val: Box<RawExpression>,
    },
    Extract {
        from: Box<RawExpression>,
        range_start: Box<RawExpression>,
        range_end: Box<RawExpression>,
    },
    Concatenate {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    GetParentValue {
        lookup: Box<RawExpression>,
        map: Box<RawExpression>,
    },
    GetFlag {
        lookup: Box<RawExpression>,
        map: Box<RawExpression>,
    },
    SemanticCast {
        kind: SemanticCastKind,
        inner: Box<RawExpression>,
    },
    ConstantInt(i128),
    RSMap,
    NotBool {
        inner: Box<RawExpression>
    },
    Add {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    Xor {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    XorBool {
        left: Box<RawExpression>,
        right: Box<RawExpression>,
    },
    Token(RawToken),
    LoadFromMemory {
        offset: Box<RawExpression>,
        size: Box<RawExpression> },
    StoreFromMemory { value: Box<RawExpression>, address: Box<RawExpression>, size: Box<RawExpression> },
    ProjectMInt { inner: Box<RawExpression> },
    MapLookup { lookup: Box<RawExpression>, map: Box<RawExpression> },
    SubMInt { left: Box<RawExpression>, right: Box<RawExpression> },
    GetRegisterValue { lookup: Box<RawExpression>, map: Box<RawExpression> },
    DecRSPInBytes { inner: Box<RawExpression> },
    FunctionCall { token: String, args: Vec<RawExpression> },
}

#[derive(Debug)]
pub enum RawToken {
    CF,
    RIP,
    RSP,
}

#[derive(Debug)]
pub enum SemanticCastKind {
    R8,
    Map,
    MInt,
    Xmm,
    R64,
}



