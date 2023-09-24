use derive_visitor::Drive;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PathElem {
    #[drive(skip)]
    name: String,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IncludedFrom {
    #[drive(skip)]
    file: String,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Loc {
    #[drive(skip)]
    offset: Option<usize>,
    #[drive(skip)]
    file: Option<String>,
    #[drive(skip)]
    line: Option<usize>,
    #[drive(skip)]
    col: Option<usize>,
    #[serde(rename = "tokLen")]
    #[drive(skip)]
    tok_len: Option<usize>,
    #[serde(rename = "includedFrom")]
    included_from: Option<IncludedFrom>,
    #[serde(rename = "spellingLoc")]
    spelling_loc: Option<Box<ASTRangeBegin>>,
    #[serde(rename = "expansionLoc")]
    expansion_loc: Option<Box<ASTRangeBegin>>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ASTRangeBegin {
    #[drive(skip)]
    offset: Option<usize>,
    #[drive(skip)]
    line: Option<usize>,
    #[drive(skip)]
    col: Option<usize>,
    #[serde(rename = "tokLen")]
    #[drive(skip)]
    tok_len: Option<usize>,
    #[serde(rename = "presumedLine")]
    #[drive(skip)]
    presumed_line: Option<usize>,
    #[serde(rename = "includedFrom")]
    included_from: Option<IncludedFrom>,
    #[serde(rename = "spellingLoc")]
    spelling_loc: Option<Box<ASTRangeBegin>>,
    #[serde(rename = "expansionLoc")]
    expansion_loc: Option<Box<ASTRangeBegin>>,
    #[drive(skip)]
    file: Option<String>,
    #[serde(rename = "isMacroArgExpansion")]
    #[drive(skip)]
    is_macro_arg_expansion: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ASTRangeEnd {
    #[drive(skip)]
    offset: Option<usize>,
    #[drive(skip)]
    line: Option<usize>,
    #[drive(skip)]
    col: Option<usize>,
    #[serde(rename = "tokLen")]
    #[drive(skip)]
    tok_len: Option<usize>,
    #[serde(rename = "presumedLine")]
    #[drive(skip)]
    presumed_line: Option<usize>,
    #[serde(rename = "includedFrom")]
    included_from: Option<IncludedFrom>,
    #[serde(rename = "spellingLoc")]
    spelling_loc: Option<Box<ASTRangeEnd>>,
    #[serde(rename = "expansionLoc")]
    expansion_loc: Option<Box<ASTRangeEnd>>,
    #[drive(skip)]
    file: Option<String>,
    #[serde(rename = "isMacroArgExpansion")]
    #[drive(skip)]
    is_macro_arg_expansion: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ASTRange {
    begin: ASTRangeBegin,
    end: ASTRangeEnd,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ASTType {
    #[serde(rename = "qualType")]
    #[drive(skip)]
    qual_type: String,
    #[serde(rename = "desugaredQualType")]
    #[drive(skip)]
    desugared_qual_type: Option<String>,
    #[serde(rename = "typeAliasDeclId")]
    #[drive(skip)]
    type_alias_decl_id: Option<String>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CopyAssign {
    #[serde(rename = "hasConstParam")]
    #[drive(skip)]
    has_const_param: bool,
    #[serde(rename = "implicitHasConstParam")]
    #[drive(skip)]
    implicit_has_const_param: bool,
    #[serde(rename = "needsImplicit")]
    #[drive(skip)]
    needs_implicit: Option<bool>,
    #[serde(rename = "userDeclared")]
    #[drive(skip)]
    user_declared: Option<bool>,
    #[drive(skip)]
    simple: Option<bool>,
    #[drive(skip)]
    trivial: Option<bool>,
    #[serde(rename = "nonTrivial")]
    #[drive(skip)]
    non_trivial: Option<bool>,
    #[serde(rename = "needsOverloadResolution")]
    #[drive(skip)]
    needs_overload_resolution: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CopyCtor {
    #[serde(rename = "hasConstParam")]
    #[drive(skip)]
    has_const_param: bool,
    #[serde(rename = "implicitHasConstParam")]
    #[drive(skip)]
    implicit_has_const_param: bool,
    #[serde(rename = "needsImplicit")]
    #[drive(skip)]
    needs_implicit: Option<bool>,
    #[serde(rename = "userDeclared")]
    #[drive(skip)]
    user_declared: Option<bool>,
    #[drive(skip)]
    simple: Option<bool>,
    #[drive(skip)]
    trivial: Option<bool>,
    #[serde(rename = "nonTrivial")]
    #[drive(skip)]
    non_trivial: Option<bool>,
    #[serde(rename = "needsOverloadResolution")]
    #[drive(skip)]
    needs_overload_resolution: Option<bool>,
    #[serde(rename = "userProvided")]
    #[drive(skip)]
    user_provided: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DefaultCtor {
    #[serde(rename = "defaultedIsConstexpr")]
    #[drive(skip)]
    defaulted_is_constexpr: Option<bool>,
    #[serde(rename = "exists")]
    #[drive(skip)]
    exists: Option<bool>,
    #[serde(rename = "isConstexpr")]
    #[drive(skip)]
    is_constexpr: Option<bool>,
    #[serde(rename = "needsImplicit")]
    #[drive(skip)]
    needs_implicit: Option<bool>,
    #[serde(rename = "userDeclared")]
    #[drive(skip)]
    user_declared: Option<bool>,
    #[drive(skip)]
    trivial: Option<bool>,
    #[serde(rename = "nonTrivial")]
    #[drive(skip)]
    non_trivial: Option<bool>,
    #[serde(rename = "needsOverloadResolution")]
    #[drive(skip)]
    needs_overload_resolution: Option<bool>,
    #[serde(rename = "userProvided")]
    #[drive(skip)]
    user_provided: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dtor {
    #[drive(skip)]
    irrelevant: Option<bool>,
    #[serde(rename = "needsImplicit")]
    #[drive(skip)]
    needs_implicit: Option<bool>,
    #[serde(rename = "userDeclared")]
    #[drive(skip)]
    user_declared: Option<bool>,
    #[drive(skip)]
    simple: Option<bool>,
    #[drive(skip)]
    trivial: Option<bool>,
    #[serde(rename = "nonTrivial")]
    #[drive(skip)]
    non_trivial: Option<bool>,
    #[serde(rename = "needsOverloadResolution")]
    #[drive(skip)]
    needs_overload_resolution: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MoveAssign {
    #[drive(skip)]
    exists: Option<bool>,
    #[serde(rename = "needsImplicit")]
    #[drive(skip)]
    needs_implicit: Option<bool>,
    #[serde(rename = "userDeclared")]
    #[drive(skip)]
    user_declared: Option<bool>,
    #[drive(skip)]
    simple: Option<bool>,
    #[drive(skip)]
    trivial: Option<bool>,
    #[serde(rename = "nonTrivial")]
    #[drive(skip)]
    non_trivial: Option<bool>,
    #[serde(rename = "needsOverloadResolution")]
    #[drive(skip)]
    needs_overload_resolution: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MoveCtor {
    #[drive(skip)]
    exists: Option<bool>,
    #[serde(rename = "needsImplicit")]
    #[drive(skip)]
    needs_implicit: Option<bool>,
    #[serde(rename = "userDeclared")]
    #[drive(skip)]
    user_declared: Option<bool>,
    #[drive(skip)]
    simple: Option<bool>,
    #[drive(skip)]
    trivial: Option<bool>,
    #[serde(rename = "nonTrivial")]
    #[drive(skip)]
    non_trivial: Option<bool>,
    #[serde(rename = "needsOverloadResolution")]
    #[drive(skip)]
    needs_overload_resolution: Option<bool>,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DefinitionData {
    #[serde(rename = "canConstDefaultInit")]
    #[drive(skip)]
    can_const_default_init: Option<bool>,
    #[serde(rename = "hasUserDeclaredConstructor")]
    #[drive(skip)]
    has_user_declared_constructor: Option<bool>,
    #[serde(rename = "canPassInRegisters")]
    #[drive(skip)]
    can_pass_in_registers: Option<bool>,
    #[serde(rename = "hasVariantMembers")]
    #[drive(skip)]
    has_variant_members: Option<bool>,
    #[serde(rename = "copyAssign")]
    copy_assign: CopyAssign,
    #[serde(rename = "copyCtor")]
    copy_ctor: CopyCtor,
    #[serde(rename = "defaultCtor")]
    default_ctor: DefaultCtor,
    #[serde(rename = "dtor")]
    dtor: Dtor,
    #[serde(rename = "hasConstexprNonCopyMoveConstructor")]
    #[drive(skip)]
    has_constexpr_non_copy_move_constructor: Option<bool>,
    #[serde(rename = "isAggregate")]
    #[drive(skip)]
    is_aggregate: Option<bool>,
    #[serde(rename = "isEmpty")]
    #[drive(skip)]
    is_empty: Option<bool>,
    #[serde(rename = "isLiteral")]
    #[drive(skip)]
    is_literal: Option<bool>,
    #[serde(rename = "isPOD")]
    #[drive(skip)]
    is_pod: Option<bool>,
    #[serde(rename = "isAbstract")]
    #[drive(skip)]
    is_abstract: Option<bool>,
    #[serde(rename = "isLambda")]
    #[drive(skip)]
    is_lambda: Option<bool>,
    #[serde(rename = "isStandardLayout")]
    #[drive(skip)]
    is_standard_layout: Option<bool>,
    #[serde(rename = "isTrivial")]
    #[drive(skip)]
    is_trivial: Option<bool>,
    #[serde(rename = "isTriviallyCopyable")]
    #[drive(skip)]
    is_trivially_copyable: Option<bool>,
    #[serde(rename = "isPolymorphic")]
    #[drive(skip)]
    is_polymorphic: Option<bool>,
    #[serde(rename = "moveAssign")]
    move_assign: MoveAssign,
    #[serde(rename = "moveCtor")]
    move_ctor: MoveCtor,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Base {
    access: Access,
    #[serde(rename = "writtenAccess")]
    written_access: Access,
    #[serde(rename = "type")]
    type_: Option<ASTType>,
}


#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum CastKind {
    Dependent,
    BitCast,
    LValueBitCast,
    LValueToRValueBitCast,
    LValueToRValue,
    NoOp,
    BaseToDerived,
    DerivedToBase,
    UncheckedDerivedToBase,
    Dynamic,
    ToUnion,
    ArrayToPointerDecay,
    FunctionToPointerDecay,
    NullToPointer,
    NullToMemberPointer,
    BaseToDerivedMemberPointer,
    DerivedToBaseMemberPointer,
    MemberPointerToBoolean,
    ReinterpretMemberPointer,
    UserDefinedConversion,
    ConstructorConversion,
    IntegralToPointer,
    PointerToIntegral,
    PointerToBoolean,
    ToVoid,
    MatrixCast,
    VectorSplat,
    IntegralCast,
    IntegralToBoolean,
    IntegralToFloating,
    FloatingToFixedPoint,
    FixedPointToFloating,
    FixedPointCast,
    FixedPointToIntegral,
    IntegralToFixedPoint,
    FixedPointToBoolean,
    FloatingToIntegral,
    FloatingToBoolean,
    BooleanToSignedIntegral,
    FloatingCast,
    CPointerToObjCPointerCast,
    BlockPointerToObjCPointerCast,
    AnyPointerToBlockPointerCast,
    ObjCObjectLValueCast,
    FloatingRealToComplex,
    FloatingComplexToReal,
    FloatingComplexToBoolean,
    FloatingComplexCast,
    FloatingComplexToIntegralComplex,
    IntegralRealToComplex,
    IntegralComplexToReal,
    IntegralComplexToBoolean,
    IntegralComplexCast,
    IntegralComplexToFloatingComplex,
    ARCProduceObject,
    ARCConsumeObject,
    ARCReclaimReturnedObject,
    ARCExtendBlockObject,
    AtomicToNonAtomic,
    NonAtomicToAtomic,
    CopyAndAutoreleaseBlockObject,
    BuiltinFnToFnPtr,
    ZeroToOCLOpaqueType,
    AddressSpaceConversion,
    IntToOCLSampler,

}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum NonOdrUseReason {
    #[serde(rename = "constant")]
    Constant,
    #[serde(rename = "unevaluated")]
    Unevaluated,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum StorageClass {
    #[serde(rename = "extern")]
    Extern,
    #[serde(rename = "static")]
    Static,
}


#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum TagUsed {
    #[serde(rename = "struct")]
    Struct,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "typename")]
    Typename,
    #[serde(rename = "union")]
    Union,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum ScopedEnumTag {
    #[serde(rename = "class")]
    Class,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum Access {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "protected")]
    Protected,
}


#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub enum ExplicitlyDefaulted {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "deleted")]
    Deleted,
}

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Empty {
    #[serde(deserialize_with = "from_hex")]
    #[drive(skip)]
    id: u64,
}


#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
pub enum ASTNode {
    DependentSizedArrayType {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    VarDecl {
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[drive(skip)]
        constexpr: Option<bool>,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        name: Option<String>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        #[drive(skip)]
        nrvo: Option<bool>,
        range: Option<ASTRange>,
        #[serde(rename = "storageClass")]
        #[drive(skip)]
        storage_class: Option<String>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[drive(skip)]
        init: Option<String>,
        #[drive(skip)]
        inline: Option<bool>,
        loc: Option<Loc>,
    },
    ImplicitValueInitExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    VarTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        name: String,
        range: Option<ASTRange>,
        loc: Option<Loc>,
    },
    ParenType {
        inner: Vec<Self>,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ParenListExpr {
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Option<Vec<Self>>,
    },
    CXXConversionDecl {
        #[drive(skip)]
        constexpr: Option<bool>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Option<Loc>,
        #[drive(skip)]
        name: String,
        range: Option<ASTRange>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXNewExpr {
        #[serde(rename = "initStyle")]
        #[drive(skip)]
        init_style: Option<String>,
        #[serde(rename = "isGlobal")]
        #[drive(skip)]
        is_global: Option<bool>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "operatorDeleteDecl")]
        operator_delete_decl: Option<Box<Self>>,
        #[serde(rename = "operatorNewDecl")]
        operator_new_decl: Option<Box<Self>>,
        #[serde(rename = "isPlacement")]
        #[drive(skip)]
        is_placement: Option<bool>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CharacterLiteral {
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[drive(skip)]
        value: i64,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    ConstantExpr {
        #[drive(skip)]
        value: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Vec<Self>,
    },
    ModeAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    CXXMethodDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        range: Option<ASTRange>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        #[serde(rename = "storageClass")]
        #[drive(skip)]
        storage_class: Option<String>,
        #[serde(rename = "explicitlyDefaulted")]
        explicitly_defaulted: Option<ExplicitlyDefaulted>,
        #[drive(skip)]
        inline: Option<bool>,
        #[drive(skip)]
        name: String,
        #[drive(skip)]
        variadic: Option<bool>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        loc: Option<Loc>,
        #[drive(skip)]
        constexpr: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "virtual")]
        #[drive(skip)]
        virtual_: Option<bool>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[drive(skip)]
        pure: Option<bool>,
        #[serde(rename = "explicitlyDeleted")]
        #[drive(skip)]
        explicitly_deleted: Option<bool>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        inner: Option<Vec<Self>>,
    },
    BuiltinAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[drive(skip)]
        inherited: Option<bool>,
        #[drive(skip)]
        implicit: bool,
    },
    TypeAliasTemplateDecl {
        range: ASTRange,
        loc: Loc,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[drive(skip)]
        name: String,
    },
    FieldDecl {
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[drive(skip)]
        name: Option<String>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "isBitfield")]
        #[drive(skip)]
        is_bitfield: Option<bool>,
        loc: Option<Loc>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
        range: Option<ASTRange>,
    },
    CXXConstructExpr {
        #[drive(skip)]
        zeroing: Option<bool>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "ctorType")]
        ctor_type: Box<ASTType>,
        #[serde(rename = "constructionKind")]
        #[drive(skip)]
        construction_kind: String,
        #[drive(skip)]
        list: Option<bool>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "hadMultipleCandidates")]
        #[drive(skip)]
        had_multiple_candidates: bool,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
    },
    ContinueStmt {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    RestrictAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    ConditionalOperator {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    OffsetOfExpr {
        inner: Option<Vec<Self>>,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXScalarValueInitExpr {
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    LinkageSpecDecl {
        loc: Loc,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "hasBraces")]
        #[drive(skip)]
        has_braces: Option<bool>,
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        language: String,
    },
    DecltypeType {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    VarTemplatePartialSpecializationDecl {
        loc: Loc,
        #[drive(skip)]
        init: String,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: String,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[drive(skip)]
        inline: bool,
        #[drive(skip)]
        name: String,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[drive(skip)]
        constexpr: bool,
    },
    PureAttr {
        range: ASTRange,
        #[drive(skip)]
        implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    FlattenAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    LValueReferenceType {
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    LoopHintAttr {
        #[drive(skip)]
        implicit: bool,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Vec<Self>,
    },
    CompoundStmt {
        inner: Option<Vec<Self>>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    TemplateSpecializationType {
        #[serde(rename = "isAlias")]
        #[drive(skip)]
        is_alias: Option<bool>,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "templateName")]
        #[drive(skip)]
        template_name: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(rename = "containsUnexpandedPack")]
        #[drive(skip)]
        contains_unexpanded_pack: Option<bool>,
    },
    Empty {
        #[drive(skip)]
        id: String
    },
    UsingDecl {
        loc: Loc,
        range: ASTRange,
        #[drive(skip)]
        name: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    RValueReferenceType {
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
    },
    UnaryTransformType {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "transformKind")]
        #[drive(skip)]
        transform_kind: String,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
    },
    SizeOfPackExpr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[drive(skip)]
        name: String,
    },
    CXXDefaultArgExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        range: ASTRange,
    },
    AccessSpecDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Loc,
        range: ASTRange,
        #[drive(skip)]
        access: String,
    },
    ElaboratedType {
        inner: Vec<Self>,
        #[serde(rename = "ownedTagDecl")]
        owned_tag_decl: Option<Box<Self>>,
        #[drive(skip)]
        qualifier: Option<String>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "containsUnexpandedPack")]
        #[drive(skip)]
        contains_unexpanded_pack: Option<bool>,
    },
    FormatAttr {
        #[drive(skip)]
        implicit: Option<bool>,
        #[drive(skip)]
        inherited: Option<bool>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    NonNullAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    CXXDependentScopeMemberExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "explicitTemplateArgs")]
        explicit_template_args: Option<Vec<Self>>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "hasExplicitTemplateArgs")]
        #[drive(skip)]
        has_explicit_template_args: Option<bool>,
        #[drive(skip)]
        member: String,
        #[serde(rename = "hasTemplateKeyword")]
        #[drive(skip)]
        has_template_keyword: Option<bool>,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isArrow")]
        #[drive(skip)]
        #[drive(skip)]
        is_arrow: bool,
    },
    IntegerLiteral {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        value: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    OpaqueValueExpr {
        inner: Option<Vec<Self>>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
    },
    CXXDestructorDecl {
        #[drive(skip)]
        inline: Option<bool>,
        loc: Option<Loc>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "virtual")]
        #[drive(skip)]
        virtual_: Option<bool>,
        #[drive(skip)]
        name: String,
        #[serde(rename = "explicitlyDefaulted")]
        explicitly_defaulted: Option<ExplicitlyDefaulted>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "explicitlyDeleted")]
        #[drive(skip)]
        explicitly_deleted: Option<bool>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        range: Option<ASTRange>,
    },
    CXXThisExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[drive(skip)]
        implicit: Option<bool>,
    },
    PackExpansionExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
    },
    ReturnStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
        range: ASTRange,
    },
    GCCAsmStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    PackExpansionType {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
    },
    CXXThrowExpr {
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    InjectedClassNameType {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        decl: Box<Self>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ClassTemplatePartialSpecializationDecl {
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[serde(rename = "definitionData")]
        definition_data: DefinitionData,
        #[drive(skip)]
        name: String,
        bases: Option<Vec<Base>>,
        range: ASTRange,
        #[serde(rename = "completeDefinition")]
        #[drive(skip)]
        complete_definition: bool,
        inner: Vec<Self>,
        #[serde(rename = "tagUsed")]
        tag_used: TagUsed,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Loc,
    },
    DefaultStmt {
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    DependentScopeDeclRefExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    FloatingLiteral {
        #[drive(skip)]
        value: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    TemplateArgument {
        #[serde(rename = "inherited from")]
        inherited_from: Option<Box<Self>>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "isExpr")]
        #[drive(skip)]
        is_expr: Option<bool>,
        decl: Option<Box<Self>>,
        #[serde(rename = "isPack")]
        #[drive(skip)]
        is_pack: Option<bool>,
        range: Option<ASTRange>,
        #[drive(skip)]
        value: Option<i64>,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
    },
    AsmLabelAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    StmtExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    UserDefinedLiteral {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    PointerAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: bool,
        range: ASTRange,
        #[drive(skip)]
        inherited: Option<bool>,
    },
    UsedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    FunctionProtoType {
        #[drive(skip)]
        cc: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[drive(skip)]
        variadic: Option<bool>,
        #[serde(rename = "const")]
        #[drive(skip)]
        const_: Option<bool>,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[drive(skip)]
        volatile: Option<bool>,
        #[serde(rename = "exceptionSpec")]
        #[drive(skip)]
        exception_spec: Option<String>,
        #[serde(rename = "refQualifier")]
        #[drive(skip)]
        ref_qualifier: Option<String>,
    },
    CXXStaticCastExpr {
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        inner: Vec<Self>,
        #[serde(rename = "conversionFunc")]
        conversion_func: Option<Box<Self>>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        range: ASTRange,
    },
    UnresolvedUsingValueDecl {
        #[serde(rename = "type")]
        type_: Option<ASTType>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: Option<ASTRange>,
        #[drive(skip)]
        name: String,
        loc: Option<Loc>,
    },
    LambdaExpr {
        inner: Vec<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    BinaryOperator {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[drive(skip)]
        opcode: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        range: ASTRange,
    },
    SwitchStmt {
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    TranslationUnitDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Option<Loc>,
        range: Option<ASTRange>,
        inner: Vec<Self>,
    },
    QualType {
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        qualifiers: String,
    },
    ClassTemplateSpecializationDecl {
        #[serde(rename = "definitionData")]
        definition_data: Option<DefinitionData>,
        #[serde(rename = "tagUsed")]
        tag_used: Option<TagUsed>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        range: Option<ASTRange>,
        loc: Option<Loc>,
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "completeDefinition")]
        #[drive(skip)]
        complete_definition: Option<bool>,
        bases: Option<Vec<Base>>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
    },
    ArrayInitLoopExpr {
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXForRangeStmt {
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    MaterializeTemporaryExpr {
        #[serde(rename = "storageDuration")]
        #[drive(skip)]
        storage_duration: String,
        #[serde(rename = "boundToLValueRef")]
        #[drive(skip)]
        bound_to_l_value_ref: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Vec<Self>,
    },
    AlwaysInlineAttr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    VarTemplateSpecializationDecl {
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[drive(skip)]
        constexpr: Option<bool>,
        inner: Option<Vec<Self>>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        init: Option<String>,
        #[drive(skip)]
        inline: Option<bool>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        range: Option<ASTRange>,
        loc: Option<Loc>,
        #[drive(skip)]
        name: String,
    },
    DependentTemplateSpecializationType {
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXDeductionGuideDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
        loc: Loc,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        variadic: Option<bool>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXPseudoDestructorExpr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    BreakStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    DeprecatedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    RecordType {
        #[serde(rename = "type")]
        type_: ASTType,
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    FunctionDecl {
        range: Option<ASTRange>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[drive(skip)]
        name: String,
        #[serde(rename = "explicitlyDeleted")]
        #[drive(skip)]
        explicitly_deleted: Option<bool>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[serde(rename = "storageClass")]
        #[drive(skip)]
        storage_class: Option<String>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[drive(skip)]
        constexpr: Option<bool>,
        #[drive(skip)]
        inline: Option<bool>,
        loc: Option<Loc>,
        #[drive(skip)]
        variadic: Option<bool>,
    },
    CXXMemberCallExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    NonTypeTemplateParmDecl {
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        depth: Option<i64>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isParameterPack")]
        #[drive(skip)]
        is_parameter_pack: Option<bool>,
        loc: Option<Loc>,
        #[drive(skip)]
        name: Option<String>,
        #[drive(skip)]
        index: Option<i64>,
        range: Option<ASTRange>,
        #[serde(rename = "defaultArg")]
        default_arg: Option<Box<Self>>,
    },
    UnaryExprOrTypeTraitExpr {
        #[drive(skip)]
        name: String,
        range: ASTRange,
        #[serde(rename = "argType")]
        arg_type: Option<ASTType>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Option<Vec<Self>>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    CXXNullPtrLiteralExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
    },
    TypedefDecl {
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[drive(skip)]
        name: String,
        loc: Option<Loc>,
        range: Option<ASTRange>,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
    },
    CXXReinterpretCastExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    MemberPointerType {
        #[serde(rename = "isFunction")]
        #[drive(skip)]
        is_function: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "isData")]
        #[drive(skip)]
        is_data: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
    },
    UsingShadowDecl {
        inner: Option<Vec<Self>>,
        target: Option<Box<Self>>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[drive(skip)]
        name: Option<String>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Option<Loc>,
        range: Option<ASTRange>,
    },
    CompoundAssignOperator {
        #[serde(rename = "computeResultType")]
        compute_result_type: Box<ASTType>,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "computeLHSType")]
        compute_lhs_type: Box<ASTType>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        opcode: String,
        range: ASTRange,
    },
    EnumConstantDecl {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Option<Loc>,
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
        range: Option<ASTRange>,
    },
    IncompleteArrayType {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
    },
    UnusedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    ArraySubscriptExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    CXXTryStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    UnresolvedLookupExpr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "usesADL")]
        #[drive(skip)]
        uses_adl: bool,
        lookups: Vec<Self>,
        #[drive(skip)]
        name: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    SubstTemplateTypeParmType {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[drive(skip)]
        pack_index: Option<i64>,
        #[drive(skip)]
        index: i64,
    },
    CaseStmt {
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXConstCastExpr {
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    OwnerAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[drive(skip)]
        implicit: bool,
        #[drive(skip)]
        inherited: Option<bool>,
    },
    EnumDecl {
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[drive(skip)]
        name: Option<String>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(rename = "scopedEnumTag")]
        scoped_enum_tag: Option<ScopedEnumTag>,
        #[serde(rename = "fixedUnderlyingType")]
        fixed_underlying_type: Option<ASTType>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: Option<ASTRange>,
        loc: Option<Loc>,
        inner: Option<Vec<Self>>,
    },
    EnumType {
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    PackedAttr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    NullStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    CXXUnresolvedConstructExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Option<Vec<Self>>,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "typeAsWritten")]
        type_as_written: Option<ASTType>,
        #[drive(skip)]
        list: Option<bool>,
    },
    DeclStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        range: ASTRange,
    },
    CXXBoolLiteralExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
        #[drive(skip)]
        value: bool,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    VisibilityAttr {
        #[drive(skip)]
        inherited: Option<bool>,
        #[drive(skip)]
        implicit: Option<bool>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    UsingType {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        decl: Box<Self>,
    },
    DoStmt {
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    FunctionTemplateDecl {
        loc: Option<Loc>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        range: Option<ASTRange>,
        #[drive(skip)]
        name: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
    },
    UsingDirectiveDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "nominatedNamespace")]
        nominated_namespace: Box<Self>,
        range: ASTRange,
        loc: Loc,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
    },
    BuiltinType {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ParmVarDecl {
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        name: Option<String>,
        range: Option<ASTRange>,
        #[serde(rename = "type")]
        type_: ASTType,
        loc: Option<Loc>,
        #[drive(skip)]
        init: Option<String>,
        #[serde(rename = "isParameterPack")]
        #[drive(skip)]
        is_parameter_pack: Option<bool>,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    DeclRefExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "referencedDecl")]
        referenced_decl: Box<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "foundReferencedDecl")]
        found_referenced_decl: Option<Box<Self>>,
        #[serde(rename = "nonOdrUseReason")]
        non_odr_use_reason: Option<NonOdrUseReason>,
        range: ASTRange,
    },
    ConstAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: Option<bool>,
        range: ASTRange,
    },
    StringLiteral {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        value: String,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    TemplateTypeParmDecl {
        #[drive(skip)]
        index: Option<i64>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Option<Loc>,
        range: Option<ASTRange>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(rename = "defaultArg")]
        default_arg: Option<Box<Self>>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "isParameterPack")]
        #[drive(skip)]
        is_parameter_pack: Option<bool>,
        #[drive(skip)]
        name: Option<String>,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        depth: Option<i64>,
        #[serde(rename = "tagUsed")]
        tag_used: Option<TagUsed>,
    },
    CXXFoldExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    ConstantArrayType {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        size: i64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ClassTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        loc: Loc,
    },
    CallExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    AllocSizeAttr {
        #[drive(skip)]
        implicit: Option<bool>,
        #[drive(skip)]
        inherited: Option<bool>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    AttributedStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Vec<Self>,
    },
    FinalAttr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    AlignedAttr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    CXXFunctionalCastExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    InitListExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Option<Vec<Self>>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        array_filler: Option<Vec<Self>>,
        range: ASTRange,
        field: Option<Box<Self>>,
    },
    TemplateTemplateParmDecl {
        #[drive(skip)]
        index: i64,
        inner: Vec<Self>,
        #[drive(skip)]
        depth: i64,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        range: ASTRange,
        loc: Loc,
        #[drive(skip)]
        name: Option<String>,
    },
    CXXBindTemporaryExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        temp: String,
        #[serde(rename = "type")]
        type_: ASTType,
        dtor: Box<Self>,
        range: ASTRange,
    },
    IfStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "hasVar")]
        #[drive(skip)]
        has_var: Option<bool>,
        #[serde(rename = "hasElse")]
        #[drive(skip)]
        has_else: Option<bool>,
        #[serde(rename = "isConstexpr")]
        #[drive(skip)]
        is_constexpr: Option<bool>,
        range: ASTRange,
    },
    ReturnsNonNullAttr {
        #[drive(skip)]
        inherited: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: bool,
        range: ASTRange,
    },
    ExprWithCleanups {
        #[serde(rename = "cleanupsHaveSideEffects")]
        #[drive(skip)]
        cleanups_have_side_effects: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    PointerType {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        inner: Vec<Self>,
    },
    CXXCatchStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        range: ASTRange,
    },
    AllocAlignAttr {
        #[drive(skip)]
        implicit: Option<bool>,
        #[drive(skip)]
        inherited: Option<bool>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    ParenExpr {
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
    },
    UnresolvedMemberExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
    },
    CXXRecordDecl {
        #[serde(rename = "completeDefinition")]
        #[drive(skip)]
        complete_definition: Option<bool>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        range: Option<ASTRange>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(rename = "tagUsed")]
        tag_used: Option<TagUsed>,
        #[serde(rename = "definitionData")]
        definition_data: Option<DefinitionData>,
        loc: Option<Loc>,
        #[drive(skip)]
        name: Option<String>,
        bases: Option<Vec<Base>>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        inner: Option<Vec<Self>>,
    },
    TypeTraitExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    DependentNameType {
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    TypedefType {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
    },
    LifetimeBoundAttr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: bool,
    },
    CXXTemporaryObjectExpr {
        inner: Option<Vec<Self>>,
        #[serde(rename = "constructionKind")]
        #[drive(skip)]
        construction_kind: String,
        range: ASTRange,
        #[drive(skip)]
        list: Option<bool>,
        #[serde(rename = "ctorType")]
        ctor_type: Box<ASTType>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "hadMultipleCandidates")]
        #[drive(skip)]
        had_multiple_candidates: bool,
        #[drive(skip)]
        zeroing: Option<bool>,
    },
    CXXCtorInitializer {
        #[serde(rename = "anyInit")]
        any_init: Option<Box<Self>>,
        inner: Vec<Self>,
        #[serde(rename = "baseInit")]
        base_init: Option<ASTType>,
    },
    CXXConstructorDecl {
        #[drive(skip)]
        variadic: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        constexpr: Option<bool>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "explicitlyDeleted")]
        #[drive(skip)]
        explicitly_deleted: Option<bool>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        loc: Option<Loc>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[drive(skip)]
        inline: Option<bool>,
        range: Option<ASTRange>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "explicitlyDefaulted")]
        explicitly_defaulted: Option<ExplicitlyDefaulted>,
        #[drive(skip)]
        name: String,
    },
    WarnUnusedResultAttr {
        range: ASTRange,
        #[drive(skip)]
        inherited: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXOperatorCallExpr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    SubstNonTypeTemplateParmExpr {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    NamespaceDecl {
        #[serde(rename = "isNested")]
        #[drive(skip)]
        is_nested: Option<bool>,
        loc: Option<Loc>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(rename = "isInline")]
        #[drive(skip)]
        is_inline: Option<bool>,
        #[serde(rename = "originalNamespace")]
        original_namespace: Option<Box<Self>>,
        #[drive(skip)]
        name: Option<String>,
        range: Option<ASTRange>,
    },
    FriendDecl {
        loc: Loc,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        inner: Option<Vec<Self>>,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
    },
    TemplateTypeParmType {
        #[drive(skip)]
        depth: i64,
        #[drive(skip)]
        index: i64,
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
        #[serde(rename = "containsUnexpandedPack")]
        #[drive(skip)]
        contains_unexpanded_pack: Option<bool>,
        #[serde(rename = "isPack")]
        #[drive(skip)]
        is_pack: Option<bool>,
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: bool,
    },
    ForStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    CXXNoexceptExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
    },
    TypeAliasDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        range: Option<ASTRange>,
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
        loc: Option<Loc>,
    },
    ImplicitCastExpr {
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "conversionFunc")]
        conversion_func: Option<Box<Self>>,
        path: Option<Vec<PathElem>>,
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "isPartOfExplicitCast")]
        #[drive(skip)]
        is_part_of_explicit_cast: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    StaticAssertDecl {
        inner: Vec<Self>,
        range: ASTRange,
        loc: Loc,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    CStyleCastExpr {
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        range: ASTRange,
    },
    IndirectFieldDecl {
        inner: Option<Vec<Self>>,
        range: ASTRange,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: bool,
        loc: Loc,
        #[drive(skip)]
        name: String,
    },
    EmptyDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        loc: Loc,
        range: ASTRange,
    },
    BuiltinTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: bool,
        #[drive(skip)]
        name: String,
        range: ASTRange,
        loc: Loc,
        inner: Vec<Self>,
    },
    ArrayInitIndexExpr {
        range: ASTRange,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    WhileStmt {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        range: ASTRange,
    },
    AbiTagAttr {
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
    },
    NoThrowAttr {
        range: ASTRange,
        #[drive(skip)]
        implicit: bool,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
    },
    UnaryOperator {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        range: ASTRange,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "canOverflow")]
        #[drive(skip)]
        can_overflow: Option<bool>,
        inner: Vec<Self>,
        #[serde(rename = "isPostfix")]
        #[drive(skip)]
        is_postfix: bool,
        #[drive(skip)]
        opcode: String,
    },
    MemberExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
        range: ASTRange,
        #[drive(skip)]
        name: String,
        #[serde(rename = "referencedMemberDecl")]
        #[drive(skip)]
        referenced_member_decl: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "nonOdrUseReason")]
        non_odr_use_reason: Option<NonOdrUseReason>,
        #[serde(deserialize_with = "from_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isArrow")]
        #[drive(skip)]
        #[drive(skip)]
        is_arrow: bool,
    },
}

fn from_hex<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
}

/*fn from_hex_optional<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: Option<String> = Deserialize::deserialize(deserializer)?;
    match s {
        None => {
            Ok(None)
        }
        Some(s) => {
            u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom).map(Some)
        }
    }
}
*/