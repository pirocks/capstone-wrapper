use derive_visitor::Drive;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;

use crate::clang_json_defs::{ASTNode, ASTType, Base, CastKind, DefinitionData, ExplicitlyDefaulted, NonOdrUseReason, ScopedEnumTag, TagUsed};

#[derive(Serialize, Deserialize, Drive, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[serde(tag = "kind")]
pub enum ASTNodeCleanedUp {
    DependentSizedArrayType {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "storageClass")]
        #[drive(skip)]
        storage_class: Option<String>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
    },
    ImplicitValueInitExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    VarTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        name: String,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ParenListExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CharacterLiteral {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[drive(skip)]
        value: i64,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    ModeAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXMethodDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        inherited: Option<bool>,
        #[drive(skip)]
        implicit: bool,
    },
    TypeAliasTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ContinueStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    RestrictAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    ConditionalOperator {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    OffsetOfExpr {
        inner: Option<Vec<Self>>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXScalarValueInitExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    LinkageSpecDecl {
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "hasBraces")]
        #[drive(skip)]
        has_braces: Option<bool>,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    VarTemplatePartialSpecializationDecl {
        #[drive(skip)]
        init: String,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: String,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
        #[drive(skip)]
        inline: bool,
        #[drive(skip)]
        name: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[drive(skip)]
        constexpr: bool,
    },
    PureAttr {
        #[drive(skip)]
        implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    FlattenAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    LValueReferenceType {
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    CompoundStmt {
        inner: Option<Vec<Self>>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        #[drive(skip)]
        name: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    RValueReferenceType {
        #[serde(rename = "isInstantiationDependent")]
        #[drive(skip)]
        is_instantiation_dependent: Option<bool>,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    AccessSpecDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    NonNullAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXDependentScopeMemberExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXDestructorDecl {
        #[drive(skip)]
        inline: Option<bool>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "mangledName")]
        #[drive(skip)]
        mangled_name: Option<String>,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
    },
    CXXThisExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(rename = "type")]
        type_: ASTType,
        #[drive(skip)]
        implicit: Option<bool>,
    },
    PackExpansionExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
    },
    ReturnStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
    },
    GCCAsmStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    PackExpansionType {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    InjectedClassNameType {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "completeDefinition")]
        #[drive(skip)]
        complete_definition: bool,
        inner: Vec<Self>,
        #[serde(rename = "tagUsed")]
        tag_used: TagUsed,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    DefaultStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    DependentScopeDeclRefExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[drive(skip)]
        value: Option<i64>,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
    },
    AsmLabelAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    StmtExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    UserDefinedLiteral {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    PointerAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: bool,
        #[drive(skip)]
        inherited: Option<bool>,
    },
    UsedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    FunctionProtoType {
        #[drive(skip)]
        cc: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    UnresolvedUsingValueDecl {
        #[serde(rename = "type")]
        type_: Option<ASTType>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
    },
    LambdaExpr {
        inner: Vec<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    SwitchStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    TranslationUnitDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    QualType {
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXForRangeStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    AlwaysInlineAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        init: Option<String>,
        #[drive(skip)]
        inline: Option<bool>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[drive(skip)]
        name: String,
    },
    DependentTemplateSpecializationType {
        #[serde(rename = "isDependent")]
        #[drive(skip)]
        is_dependent: bool,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
        #[drive(skip)]
        variadic: Option<bool>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXPseudoDestructorExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    DeprecatedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    RecordType {
        #[serde(rename = "type")]
        type_: ASTType,
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    FunctionDecl {
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isUsed")]
        #[drive(skip)]
        is_used: Option<bool>,
        #[drive(skip)]
        constexpr: Option<bool>,
        #[drive(skip)]
        inline: Option<bool>,
        #[drive(skip)]
        variadic: Option<bool>,
    },
    CXXMemberCallExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
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
        #[drive(skip)]
        name: Option<String>,
        #[drive(skip)]
        index: Option<i64>,
        #[serde(rename = "defaultArg")]
        default_arg: Option<Box<Self>>,
    },
    UnaryExprOrTypeTraitExpr {
        #[drive(skip)]
        name: String,
        #[serde(rename = "argType")]
        arg_type: Option<ASTType>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    TypedefDecl {
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[drive(skip)]
        name: String,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        opcode: String,
    },
    EnumConstantDecl {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
    },
    IncompleteArrayType {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    ArraySubscriptExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    CXXTryStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    UnresolvedLookupExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXConstCastExpr {
        inner: Vec<Self>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "castKind")]
        cast_kind: CastKind,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    OwnerAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Option<Vec<Self>>,
    },
    EnumType {
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    PackedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    NullStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXUnresolvedConstructExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Option<Vec<Self>>,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "typeAsWritten")]
        type_as_written: Option<ASTType>,
        #[drive(skip)]
        list: Option<bool>,
    },
    DeclStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    CXXBoolLiteralExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    UsingType {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        decl: Box<Self>,
    },
    DoStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    FunctionTemplateDecl {
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        inner: Option<Vec<Self>>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[drive(skip)]
        name: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
    },
    UsingDirectiveDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "nominatedNamespace")]
        nominated_namespace: Box<Self>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
    },
    BuiltinType {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "type")]
        type_: ASTType,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    DeclRefExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
    },
    ConstAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: Option<bool>,
    },
    StringLiteral {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        value: String,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    TemplateTypeParmDecl {
        #[drive(skip)]
        index: Option<i64>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    ConstantArrayType {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        size: i64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    ClassTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        name: String,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        inner: Vec<Self>,
        #[serde(rename = "parentDeclContextId")]
        #[drive(skip)]
        parent_decl_context_id: Option<String>,
    },
    CallExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    AttributedStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    FinalAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    AlignedAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        array_filler: Option<Vec<Self>>,
        field: Option<Box<Self>>,
    },
    TemplateTemplateParmDecl {
        #[drive(skip)]
        index: i64,
        inner: Vec<Self>,
        #[drive(skip)]
        depth: i64,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[drive(skip)]
        name: Option<String>,
    },
    CXXBindTemporaryExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        temp: String,
        #[serde(rename = "type")]
        type_: ASTType,
        dtor: Box<Self>,
    },
    IfStmt {
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
    },
    ReturnsNonNullAttr {
        #[drive(skip)]
        inherited: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[drive(skip)]
        implicit: bool,
    },
    ExprWithCleanups {
        #[serde(rename = "cleanupsHaveSideEffects")]
        #[drive(skip)]
        cleanups_have_side_effects: Option<bool>,
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    AllocAlignAttr {
        #[drive(skip)]
        implicit: Option<bool>,
        #[drive(skip)]
        inherited: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    ParenExpr {
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: ASTType,
    },
    CXXRecordDecl {
        #[serde(rename = "completeDefinition")]
        #[drive(skip)]
        complete_definition: Option<bool>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "previousDecl")]
        #[drive(skip)]
        previous_decl: Option<String>,
        #[serde(rename = "tagUsed")]
        tag_used: Option<TagUsed>,
        #[serde(rename = "definitionData")]
        definition_data: Option<DefinitionData>,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    TypedefType {
        #[serde(rename = "type")]
        type_: ASTType,
        inner: Vec<Self>,
        decl: Box<Self>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(rename = "explicitlyDefaulted")]
        explicitly_defaulted: Option<ExplicitlyDefaulted>,
        #[drive(skip)]
        name: String,
    },
    WarnUnusedResultAttr {
        #[drive(skip)]
        inherited: Option<bool>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXOperatorCallExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
    },
    FriendDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    CXXNoexceptExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
        inner: Vec<Self>,
    },
    TypeAliasDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "type")]
        type_: Option<ASTType>,
        #[serde(rename = "isReferenced")]
        #[drive(skip)]
        is_referenced: Option<bool>,
        #[drive(skip)]
        name: String,
        inner: Option<Vec<Self>>,
    },
    ImplicitCastExpr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
        #[serde(rename = "conversionFunc")]
        conversion_func: Option<Box<Self>>,
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
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    IndirectFieldDecl {
        inner: Option<Vec<Self>>,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: bool,
        #[drive(skip)]
        name: String,
    },
    EmptyDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    BuiltinTemplateDecl {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isImplicit")]
        #[drive(skip)]
        is_implicit: bool,
        #[drive(skip)]
        name: String,
        inner: Vec<Self>,
    },
    ArrayInitIndexExpr {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "valueCategory")]
        #[drive(skip)]
        value_category: String,
    },
    WhileStmt {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        inner: Vec<Self>,
    },
    AbiTagAttr {
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    NoThrowAttr {
        #[drive(skip)]
        implicit: bool,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
    },
    UnaryOperator {
        #[serde(rename = "type")]
        type_: ASTType,
        #[serde(deserialize_with = "from_hex")]
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
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
        #[serde(serialize_with = "to_hex")]
        #[drive(skip)]
        id: u64,
        #[serde(rename = "isArrow")]
        #[drive(skip)]
        #[drive(skip)]
        is_arrow: bool,
    },
}

impl ASTNodeCleanedUp {
    fn from_unclean_vec(unclean: Vec<ASTNode>) -> Vec<ASTNodeCleanedUp> {
        unclean.into_iter().map(|unclean| ASTNodeCleanedUp::from_unclean(unclean)).collect()
    }

    fn from_unclean_vec_option(unclean: Option<Vec<ASTNode>>) -> Option<Vec<ASTNodeCleanedUp>> {
        unclean.map(|unclean| Self::from_unclean_vec(unclean))
    }

    fn from_unclean_option_box(unclean: Option<Box<ASTNode>>) -> Option<Box<ASTNodeCleanedUp>> {
        unclean.map(|unclean| Box::new(Self::from_unclean(*unclean)))
    }

    pub fn from_unclean(unclean: ASTNode) -> ASTNodeCleanedUp {
        match unclean {
            ASTNode::DependentSizedArrayType { id, inner, is_dependent, is_instantiation_dependent, type_, .. } => {
                ASTNodeCleanedUp::DependentSizedArrayType { id, inner: Self::from_unclean_vec(inner), is_dependent, is_instantiation_dependent, type_ }
            }
            ASTNode::VarDecl { is_used, constexpr, inner, name, parent_decl_context_id, previous_decl, mangled_name, nrvo, storage_class, type_, id, is_implicit, is_referenced, init, inline, .. } => {
                ASTNodeCleanedUp::VarDecl { is_used, constexpr, inner: Self::from_unclean_vec_option(inner), name, parent_decl_context_id, previous_decl, mangled_name, nrvo, storage_class, type_, id, is_implicit, is_referenced, init, inline }
            }
            ASTNode::ImplicitValueInitExpr { type_, value_category, id, .. } => {
                ASTNodeCleanedUp::ImplicitValueInitExpr { type_, value_category, id }
            }
            ASTNode::VarTemplateDecl { id, inner, name, .. } => {
                ASTNodeCleanedUp::VarTemplateDecl { id, inner: Self::from_unclean_vec_option(inner), name }
            }
            ASTNode::ParenType { inner, is_dependent, is_instantiation_dependent, id, type_, .. } => {
                ASTNodeCleanedUp::ParenType { inner: Self::from_unclean_vec(inner), is_dependent, is_instantiation_dependent, id, type_ }
            }
            ASTNode::ParenListExpr { type_, id, value_category, inner, .. } => {
                ASTNodeCleanedUp::ParenListExpr { type_, id, value_category, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::CXXConversionDecl { constexpr, mangled_name, inner, is_used, id, name, type_, .. } => {
                ASTNodeCleanedUp::CXXConversionDecl { constexpr, mangled_name, inner: Self::from_unclean_vec_option(inner), is_used, id, name, type_ }
            }
            ASTNode::CXXNewExpr { init_style, is_global, inner, operator_delete_decl, operator_new_decl, is_placement, value_category, id, type_, .. } => {
                ASTNodeCleanedUp::CXXNewExpr { init_style, is_global, inner: Self::from_unclean_vec_option(inner), operator_delete_decl: Self::from_unclean_option_box(operator_delete_decl), operator_new_decl: Self::from_unclean_option_box(operator_new_decl), is_placement, value_category, id, type_ }
            }
            ASTNode::CharacterLiteral { type_, value_category, value, id, .. } => {
                ASTNodeCleanedUp::CharacterLiteral { type_, value_category, value, id }
            }
            ASTNode::ConstantExpr { value, type_, value_category, id, inner, .. } => {
                ASTNodeCleanedUp::ConstantExpr { value, type_, value_category, id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::ModeAttr { id, .. } => {
                ASTNodeCleanedUp::ModeAttr { id }
            }
            ASTNode::CXXMethodDecl { id, is_referenced, mangled_name, storage_class, explicitly_defaulted, inline, name, variadic, is_implicit, constexpr, type_, virtual_, previous_decl, pure, explicitly_deleted, parent_decl_context_id, is_used, inner, .. } => {
                ASTNodeCleanedUp::CXXMethodDecl { id, is_referenced, mangled_name, storage_class, explicitly_defaulted, inline, name, variadic, is_implicit, constexpr, type_, virtual_, previous_decl, pure, explicitly_deleted, parent_decl_context_id, is_used, inner : Self::from_unclean_vec_option(inner) }
            }
            ASTNode::BuiltinAttr { id, inherited, implicit, .. } => {
                ASTNodeCleanedUp::BuiltinAttr { id, inherited, implicit }
            }
            ASTNode::TypeAliasTemplateDecl { id, inner, name, .. } => {
                ASTNodeCleanedUp::TypeAliasTemplateDecl { id, inner: Self::from_unclean_vec(inner), name }
            }
            ASTNode::FieldDecl { is_referenced, name, is_implicit, is_bitfield, type_, id, inner, .. } => {
                ASTNodeCleanedUp::FieldDecl { is_referenced, name, is_implicit, is_bitfield, type_, id, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::CXXConstructExpr { zeroing, inner, ctor_type, construction_kind, list, value_category, had_multiple_candidates, id, type_, .. } => {
                ASTNodeCleanedUp::CXXConstructExpr { zeroing, inner: Self::from_unclean_vec_option(inner), ctor_type, construction_kind, list, value_category, had_multiple_candidates, id, type_ }
            }
            ASTNode::ContinueStmt { id, .. } => {
                ASTNodeCleanedUp::ContinueStmt { id }
            }
            ASTNode::RestrictAttr { id, .. } => {
                ASTNodeCleanedUp::RestrictAttr { id }
            }
            ASTNode::ConditionalOperator { value_category, type_, inner, id, .. } => {
                ASTNodeCleanedUp::ConditionalOperator { value_category, type_, inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::OffsetOfExpr { inner, value_category, type_, id, .. } => {
                ASTNodeCleanedUp::OffsetOfExpr { inner: Self::from_unclean_vec_option(inner), value_category, type_, id }
            }
            ASTNode::CXXScalarValueInitExpr { type_, id, value_category, .. } => {
                ASTNodeCleanedUp::CXXScalarValueInitExpr { type_, id, value_category }
            }
            ASTNode::LinkageSpecDecl { is_implicit, has_braces, inner, id, language, .. } => {
                ASTNodeCleanedUp::LinkageSpecDecl { is_implicit, has_braces, inner: Self::from_unclean_vec(inner), id, language }
            }
            ASTNode::DecltypeType { type_, is_dependent, is_instantiation_dependent, id, inner, .. } => {
                ASTNodeCleanedUp::DecltypeType { type_, is_dependent, is_instantiation_dependent, id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::VarTemplatePartialSpecializationDecl { init, mangled_name, inner, id, parent_decl_context_id, inline, name, type_, constexpr, .. } => {
                ASTNodeCleanedUp::VarTemplatePartialSpecializationDecl { init, mangled_name, inner: Self::from_unclean_vec(inner), id, parent_decl_context_id, inline, name, type_, constexpr }
            }
            ASTNode::PureAttr { implicit, id, .. } => {
                ASTNodeCleanedUp::PureAttr { implicit, id }
            }
            ASTNode::FlattenAttr { id, .. } => {
                ASTNodeCleanedUp::FlattenAttr { id }
            }
            ASTNode::LValueReferenceType { is_instantiation_dependent, id, is_dependent, inner, type_, .. } => {
                ASTNodeCleanedUp::LValueReferenceType { is_instantiation_dependent, id, is_dependent, inner : Self::from_unclean_vec(inner), type_ }
            }
            ASTNode::LoopHintAttr { implicit, id, inner, .. } => {
                ASTNodeCleanedUp::LoopHintAttr { implicit, id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::CompoundStmt { inner, id, .. } => {
                ASTNodeCleanedUp::CompoundStmt { inner: Self::from_unclean_vec_option(inner), id }
            }
            ASTNode::TemplateSpecializationType { is_alias, is_dependent, inner, type_, template_name, id, is_instantiation_dependent, contains_unexpanded_pack, .. } => {
                ASTNodeCleanedUp::TemplateSpecializationType { is_alias, is_dependent, inner: Self::from_unclean_vec(inner), type_, template_name, id, is_instantiation_dependent, contains_unexpanded_pack }
            }
            ASTNode::Empty { id, .. } => {
                ASTNodeCleanedUp::Empty { id }
            }
            ASTNode::UsingDecl { name, id, .. } => {
                ASTNodeCleanedUp::UsingDecl { name, id }
            }
            ASTNode::RValueReferenceType { is_instantiation_dependent, inner, id, type_, is_dependent, .. } => {
                ASTNodeCleanedUp::RValueReferenceType { is_instantiation_dependent, inner : Self::from_unclean_vec(inner), id, type_, is_dependent }
            }
            ASTNode::UnaryTransformType { type_, transform_kind, inner, id, is_instantiation_dependent, is_dependent, .. } => {
                ASTNodeCleanedUp::UnaryTransformType { type_, transform_kind, inner : Self::from_unclean_vec(inner), id, is_instantiation_dependent, is_dependent }
            }
            ASTNode::SizeOfPackExpr { id, type_, value_category, name, .. } => {
                ASTNodeCleanedUp::SizeOfPackExpr { id, type_, value_category, name }
            }
            ASTNode::CXXDefaultArgExpr { id, type_, value_category, .. } => {
                ASTNodeCleanedUp::CXXDefaultArgExpr { id, type_, value_category }
            }
            ASTNode::AccessSpecDecl { id, access, .. } => {
                ASTNodeCleanedUp::AccessSpecDecl { id, access }
            }
            ASTNode::ElaboratedType { inner, owned_tag_decl, qualifier, type_, is_dependent, is_instantiation_dependent, id, contains_unexpanded_pack, .. } => {
                ASTNodeCleanedUp::ElaboratedType { inner: Self::from_unclean_vec(inner), owned_tag_decl: Self::from_unclean_option_box(owned_tag_decl), qualifier, type_, is_dependent, is_instantiation_dependent, id, contains_unexpanded_pack }
            }
            ASTNode::FormatAttr { implicit, inherited, id, .. } => {
                ASTNodeCleanedUp::FormatAttr { implicit, inherited, id }
            }
            ASTNode::NonNullAttr { id, .. } => {
                ASTNodeCleanedUp::NonNullAttr { id }
            }
            ASTNode::CXXDependentScopeMemberExpr { id, explicit_template_args, inner, has_explicit_template_args, member, has_template_keyword, value_category, type_, is_arrow, .. } => {
                ASTNodeCleanedUp::CXXDependentScopeMemberExpr { id, explicit_template_args: Self::from_unclean_vec_option(explicit_template_args), inner: Self::from_unclean_vec_option(inner), has_explicit_template_args, member, has_template_keyword, value_category, type_, is_arrow }
            }
            ASTNode::IntegerLiteral { id, value, type_, value_category, .. } => {
                ASTNodeCleanedUp::IntegerLiteral { id, value, type_, value_category }
            }
            ASTNode::OpaqueValueExpr { inner, value_category, id, type_, .. } => {
                ASTNodeCleanedUp::OpaqueValueExpr { inner: Self::from_unclean_vec_option(inner), value_category, id, type_ }
            }
            ASTNode::CXXDestructorDecl { inline, inner, mangled_name, is_used, id, virtual_, name, explicitly_defaulted, is_referenced, type_, explicitly_deleted, is_implicit, .. } => {
                ASTNodeCleanedUp::CXXDestructorDecl { inline, inner: Self::from_unclean_vec_option(inner), mangled_name, is_used, id, virtual_, name, explicitly_defaulted, is_referenced, type_, explicitly_deleted, is_implicit }
            }
            ASTNode::CXXThisExpr { id, value_category, type_, implicit, .. } => {
                ASTNodeCleanedUp::CXXThisExpr { id, value_category, type_, implicit }
            }
            ASTNode::PackExpansionExpr { id, type_, value_category, inner, .. } => {
                ASTNodeCleanedUp::PackExpansionExpr { id, type_, value_category, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::ReturnStmt { id, inner, .. } => {
                ASTNodeCleanedUp::ReturnStmt { id, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::GCCAsmStmt { id, .. } => {
                ASTNodeCleanedUp::GCCAsmStmt { id }
            }
            ASTNode::PackExpansionType { id, inner, type_, is_instantiation_dependent, is_dependent, .. } => {
                ASTNodeCleanedUp::PackExpansionType { id, inner : Self::from_unclean_vec(inner), type_, is_instantiation_dependent, is_dependent }
            }
            ASTNode::CXXThrowExpr { value_category, id, type_, .. } => {
                ASTNodeCleanedUp::CXXThrowExpr { value_category, id, type_ }
            }
            ASTNode::InjectedClassNameType { id, decl, is_instantiation_dependent, is_dependent, type_, .. } => {
                ASTNodeCleanedUp::InjectedClassNameType { id, decl: Box::new(Self::from_unclean(*decl)), is_instantiation_dependent, is_dependent, type_ }
            }
            ASTNode::ClassTemplatePartialSpecializationDecl { parent_decl_context_id, definition_data, name, bases, complete_definition, inner, tag_used, id, .. } => {
                ASTNodeCleanedUp::ClassTemplatePartialSpecializationDecl { parent_decl_context_id, definition_data, name, bases, complete_definition, inner : Self::from_unclean_vec(inner), tag_used, id }
            }
            ASTNode::DefaultStmt { inner, id, .. } => {
                ASTNodeCleanedUp::DefaultStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::DependentScopeDeclRefExpr { type_, id, value_category, .. } => {
                ASTNodeCleanedUp::DependentScopeDeclRefExpr { type_, id, value_category }
            }
            ASTNode::FloatingLiteral { value, type_, id, value_category, .. } => {
                ASTNodeCleanedUp::FloatingLiteral { value, type_, id, value_category }
            }
            ASTNode::TemplateArgument { inherited_from, inner, is_expr, decl, is_pack, value, type_, .. } => {
                ASTNodeCleanedUp::TemplateArgument { inherited_from: Self::from_unclean_option_box(inherited_from), inner: Self::from_unclean_vec_option(inner), is_expr, decl: Self::from_unclean_option_box(decl), is_pack, value, type_ }
            }
            ASTNode::AsmLabelAttr { id, .. } => {
                ASTNodeCleanedUp::AsmLabelAttr { id }
            }
            ASTNode::StmtExpr { id, value_category, inner, type_, .. } => {
                ASTNodeCleanedUp::StmtExpr { id, value_category, inner : Self::from_unclean_vec(inner), type_ }
            }
            ASTNode::UserDefinedLiteral { id, inner, type_, value_category, .. } => {
                ASTNodeCleanedUp::UserDefinedLiteral { id, inner : Self::from_unclean_vec(inner), type_, value_category }
            }
            ASTNode::PointerAttr { id, implicit, inherited, .. } => {
                ASTNodeCleanedUp::PointerAttr { id, implicit, inherited }
            }
            ASTNode::UsedAttr { id, .. } => {
                ASTNodeCleanedUp::UsedAttr { id }
            }
            ASTNode::FunctionProtoType { cc, id, inner, is_instantiation_dependent, type_, variadic, const_, is_dependent, volatile, exception_spec, ref_qualifier, .. } => {
                ASTNodeCleanedUp::FunctionProtoType { cc, id, inner: Self::from_unclean_vec(inner), is_instantiation_dependent, type_, variadic, const_, is_dependent, volatile, exception_spec, ref_qualifier }
            }
            ASTNode::CXXStaticCastExpr { cast_kind, inner, conversion_func, id, type_, value_category, .. } => {
                ASTNodeCleanedUp::CXXStaticCastExpr { cast_kind, inner: Self::from_unclean_vec(inner), conversion_func: Self::from_unclean_option_box(conversion_func), id, type_, value_category }
            }
            ASTNode::UnresolvedUsingValueDecl { type_, id, name, .. } => {
                ASTNodeCleanedUp::UnresolvedUsingValueDecl { type_, id, name }
            }
            ASTNode::LambdaExpr { inner, value_category, id, type_, .. } => {
                ASTNodeCleanedUp::LambdaExpr { inner: Self::from_unclean_vec(inner), value_category, id, type_ }
            }
            ASTNode::BinaryOperator { type_, value_category, opcode, id, inner, .. } => {
                ASTNodeCleanedUp::BinaryOperator { type_, value_category, opcode, id, inner : Self::from_unclean_vec(inner) }
            }
            ASTNode::SwitchStmt { inner, id, .. } => {
                ASTNodeCleanedUp::SwitchStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::TranslationUnitDecl { id, inner, .. } => {
                ASTNodeCleanedUp::TranslationUnitDecl { id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::QualType { inner, type_, id, qualifiers, .. } => {
                ASTNodeCleanedUp::QualType { inner: Self::from_unclean_vec(inner), type_, id, qualifiers }
            }
            ASTNode::ClassTemplateSpecializationDecl { definition_data, tag_used, previous_decl, name, inner, id, complete_definition, bases, parent_decl_context_id, .. } => {
                ASTNodeCleanedUp::ClassTemplateSpecializationDecl { definition_data, tag_used, previous_decl, name, inner: Self::from_unclean_vec_option(inner), id, complete_definition, bases, parent_decl_context_id }
            }
            ASTNode::ArrayInitLoopExpr { value_category, id, inner, type_, .. } => {
                ASTNodeCleanedUp::ArrayInitLoopExpr { value_category, id, inner: Self::from_unclean_vec(inner), type_ }
            }
            ASTNode::CXXForRangeStmt { inner, id, .. } => {
                ASTNodeCleanedUp::CXXForRangeStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::MaterializeTemporaryExpr { storage_duration, bound_to_l_value_ref, type_, value_category, id, inner, .. } => {
                ASTNodeCleanedUp::MaterializeTemporaryExpr { storage_duration, bound_to_l_value_ref, type_, value_category, id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::AlwaysInlineAttr { id, .. } => {
                ASTNodeCleanedUp::AlwaysInlineAttr { id }
            }
            ASTNode::VarTemplateSpecializationDecl { mangled_name, type_, constexpr, inner, id, init, inline, is_referenced, name, .. } => {
                ASTNodeCleanedUp::VarTemplateSpecializationDecl { mangled_name, type_, constexpr, inner: Self::from_unclean_vec_option(inner), id, init, inline, is_referenced, name }
            }
            ASTNode::DependentTemplateSpecializationType { is_dependent, id, is_instantiation_dependent, type_, .. } => {
                ASTNodeCleanedUp::DependentTemplateSpecializationType { is_dependent, id, is_instantiation_dependent, type_ }
            }
            ASTNode::CXXDeductionGuideDecl { id, name, inner, variadic, is_implicit, type_, .. } => {
                ASTNodeCleanedUp::CXXDeductionGuideDecl { id, name, inner: Self::from_unclean_vec_option(inner), variadic, is_implicit, type_ }
            }
            ASTNode::CXXPseudoDestructorExpr { id, inner, value_category, type_, .. } => {
                ASTNodeCleanedUp::CXXPseudoDestructorExpr { id, inner : Self::from_unclean_vec(inner), value_category, type_ }
            }
            ASTNode::BreakStmt { id, .. } => {
                ASTNodeCleanedUp::BreakStmt { id }
            }
            ASTNode::DeprecatedAttr { id, .. } => {
                ASTNodeCleanedUp::DeprecatedAttr { id }
            }
            ASTNode::RecordType { type_, decl, id, .. } => {
                ASTNodeCleanedUp::RecordType { type_, decl: Box::new(Self::from_unclean(*decl)), id }
            }
            ASTNode::FunctionDecl { mangled_name, type_, is_referenced, name, explicitly_deleted, inner, parent_decl_context_id, storage_class, is_implicit, previous_decl, id, is_used, constexpr, inline, variadic, .. } => {
                ASTNodeCleanedUp::FunctionDecl { mangled_name, type_, is_referenced, name, explicitly_deleted, inner: Self::from_unclean_vec_option(inner), parent_decl_context_id, storage_class, is_implicit, previous_decl, id, is_used, constexpr, inline, variadic }
            }
            ASTNode::CXXMemberCallExpr { id, inner, type_, value_category, .. } => {
                ASTNodeCleanedUp::CXXMemberCallExpr { id, inner: Self::from_unclean_vec(inner), type_, value_category }
            }
            ASTNode::NonTypeTemplateParmDecl { inner, depth, id, is_implicit, is_referenced, type_, is_parameter_pack, name, index, default_arg, .. } => {
                ASTNodeCleanedUp::NonTypeTemplateParmDecl { inner: Self::from_unclean_vec_option(inner), depth, id, is_implicit, is_referenced, type_, is_parameter_pack, name, index, default_arg: Self::from_unclean_option_box(default_arg) }
            }
            ASTNode::UnaryExprOrTypeTraitExpr { name, arg_type, id, type_, inner, value_category, .. } => {
                ASTNodeCleanedUp::UnaryExprOrTypeTraitExpr { name, arg_type, id, type_, inner: Self::from_unclean_vec_option(inner), value_category }
            }
            ASTNode::CXXNullPtrLiteralExpr { value_category, id, type_, .. } => {
                ASTNodeCleanedUp::CXXNullPtrLiteralExpr { value_category, id, type_ }
            }
            ASTNode::TypedefDecl { previous_decl, name, type_, inner, is_implicit, id, is_referenced, .. } => {
                ASTNodeCleanedUp::TypedefDecl { previous_decl, name, type_, inner: Self::from_unclean_vec_option(inner), is_implicit, id, is_referenced }
            }
            ASTNode::CXXReinterpretCastExpr { type_, inner, cast_kind, id, value_category, .. } => {
                ASTNodeCleanedUp::CXXReinterpretCastExpr { type_, inner: Self::from_unclean_vec(inner), cast_kind, id, value_category }
            }
            ASTNode::MemberPointerType { is_function, id, inner, is_instantiation_dependent, is_data, type_, is_dependent, .. } => {
                ASTNodeCleanedUp::MemberPointerType { is_function, id, inner: Self::from_unclean_vec(inner), is_instantiation_dependent, is_data, type_, is_dependent }
            }
            ASTNode::UsingShadowDecl { inner, target, is_implicit, previous_decl, name, id, .. } => {
                ASTNodeCleanedUp::UsingShadowDecl { inner: Self::from_unclean_vec_option(inner), target: Self::from_unclean_option_box(target), is_implicit, previous_decl, name, id }
            }
            ASTNode::CompoundAssignOperator { compute_result_type, inner, type_, compute_lhs_type, value_category, id, opcode, .. } => {
                ASTNodeCleanedUp::CompoundAssignOperator { compute_result_type, inner: Self::from_unclean_vec(inner), type_, compute_lhs_type, value_category, id, opcode }
            }
            ASTNode::EnumConstantDecl { type_, is_referenced, id, name, inner, .. } => {
                ASTNodeCleanedUp::EnumConstantDecl { type_, is_referenced, id, name, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::IncompleteArrayType { inner, id, is_instantiation_dependent, type_, is_dependent, .. } => {
                ASTNodeCleanedUp::IncompleteArrayType { inner: Self::from_unclean_vec(inner), id, is_instantiation_dependent, type_, is_dependent }
            }
            ASTNode::UnusedAttr { id, .. } => {
                ASTNodeCleanedUp::UnusedAttr { id }
            }
            ASTNode::ArraySubscriptExpr { type_, id, inner, value_category, .. } => {
                ASTNodeCleanedUp::ArraySubscriptExpr { type_, id, inner: Self::from_unclean_vec(inner), value_category }
            }
            ASTNode::CXXTryStmt { inner, id, .. } => {
                ASTNodeCleanedUp::CXXTryStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::UnresolvedLookupExpr { id, uses_adl, lookups, name, type_, value_category, .. } => {
                ASTNodeCleanedUp::UnresolvedLookupExpr { id, uses_adl, lookups: Self::from_unclean_vec(lookups), name, type_, value_category }
            }
            ASTNode::SubstTemplateTypeParmType { type_, id, inner, is_dependent, is_instantiation_dependent, pack_index, index, .. } => {
                ASTNodeCleanedUp::SubstTemplateTypeParmType { type_, id, inner: Self::from_unclean_vec(inner), is_dependent, is_instantiation_dependent, pack_index, index }
            }
            ASTNode::CaseStmt { inner, id, .. } => {
                ASTNodeCleanedUp::CaseStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::CXXConstCastExpr { inner, value_category, id, cast_kind, type_, .. } => {
                ASTNodeCleanedUp::CXXConstCastExpr { inner: Self::from_unclean_vec(inner), value_category, id, cast_kind, type_ }
            }
            ASTNode::OwnerAttr { id, implicit, inherited, .. } => {
                ASTNodeCleanedUp::OwnerAttr { id, implicit, inherited }
            }
            ASTNode::EnumDecl { is_referenced, name, previous_decl, scoped_enum_tag, fixed_underlying_type, id, inner, .. } => {
                ASTNodeCleanedUp::EnumDecl { is_referenced, name, previous_decl, scoped_enum_tag, fixed_underlying_type, id, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::EnumType { decl, id, type_, .. } => {
                ASTNodeCleanedUp::EnumType { decl: Box::new(Self::from_unclean(*decl)), id, type_ }
            }
            ASTNode::PackedAttr { id, .. } => {
                ASTNodeCleanedUp::PackedAttr { id }
            }
            ASTNode::NullStmt { id, .. } => {
                ASTNodeCleanedUp::NullStmt { id }
            }
            ASTNode::CXXUnresolvedConstructExpr { type_, inner, value_category, id, type_as_written, list, .. } => {
                ASTNodeCleanedUp::CXXUnresolvedConstructExpr { type_, inner : Self::from_unclean_vec_option(inner), value_category, id, type_as_written, list }
            }
            ASTNode::DeclStmt { id, inner, .. } => {
                ASTNodeCleanedUp::DeclStmt { id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::CXXBoolLiteralExpr { id, type_, value, value_category, .. } => {
                ASTNodeCleanedUp::CXXBoolLiteralExpr { id, type_, value, value_category }
            }
            ASTNode::VisibilityAttr { inherited, implicit, id, .. } => {
                ASTNodeCleanedUp::VisibilityAttr { inherited, implicit, id }
            }
            ASTNode::UsingType { type_, inner, id, decl, .. } => {
                ASTNodeCleanedUp::UsingType { type_, inner: Self::from_unclean_vec(inner), id, decl: Box::new(Self::from_unclean(*decl)) }
            }
            ASTNode::DoStmt { inner, id, .. } => {
                ASTNodeCleanedUp::DoStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::FunctionTemplateDecl { previous_decl, inner, is_implicit, name, id, parent_decl_context_id, .. } => {
                ASTNodeCleanedUp::FunctionTemplateDecl { previous_decl, inner: Self::from_unclean_vec_option(inner), is_implicit, name, id, parent_decl_context_id }
            }
            ASTNode::UsingDirectiveDecl { id, nominated_namespace, is_implicit, .. } => {
                ASTNodeCleanedUp::UsingDirectiveDecl { id, nominated_namespace: Box::new(Self::from_unclean(*nominated_namespace)), is_implicit }
            }
            ASTNode::BuiltinType { id, type_, .. } => {
                ASTNodeCleanedUp::BuiltinType { id, type_ }
            }
            ASTNode::ParmVarDecl { is_referenced, inner, name, type_, init, is_parameter_pack, is_used, is_implicit, id, .. } => {
                ASTNodeCleanedUp::ParmVarDecl { is_referenced, inner: Self::from_unclean_vec_option(inner), name, type_, init, is_parameter_pack, is_used, is_implicit, id }
            }
            ASTNode::DeclRefExpr { id, type_, referenced_decl, value_category, found_referenced_decl, non_odr_use_reason, .. } => {
                ASTNodeCleanedUp::DeclRefExpr { id, type_, referenced_decl: Box::new(Self::from_unclean(*referenced_decl)), value_category, found_referenced_decl: Self::from_unclean_option_box(found_referenced_decl), non_odr_use_reason }
            }
            ASTNode::ConstAttr { id, implicit, .. } => {
                ASTNodeCleanedUp::ConstAttr { id, implicit }
            }
            ASTNode::StringLiteral { type_, id, value, value_category, .. } => {
                ASTNodeCleanedUp::StringLiteral { type_, id, value, value_category }
            }
            ASTNode::TemplateTypeParmDecl { index, id, is_referenced, default_arg, is_implicit, is_parameter_pack, name, inner, depth, tag_used, .. } => {
                ASTNodeCleanedUp::TemplateTypeParmDecl { index, id, is_referenced, default_arg: Self::from_unclean_option_box(default_arg), is_implicit, is_parameter_pack, name, inner : Self::from_unclean_vec_option(inner), depth, tag_used }
            }
            ASTNode::CXXFoldExpr { type_, inner, id, value_category, .. } => {
                ASTNodeCleanedUp::CXXFoldExpr { type_, inner: Self::from_unclean_vec(inner), id, value_category }
            }
            ASTNode::ConstantArrayType { inner, id, size, type_, .. } => {
                ASTNodeCleanedUp::ConstantArrayType { inner: Self::from_unclean_vec(inner), id, size, type_ }
            }
            ASTNode::ClassTemplateDecl { id, name, previous_decl, inner, parent_decl_context_id, .. } => {
                ASTNodeCleanedUp::ClassTemplateDecl { id, name, previous_decl, inner: Self::from_unclean_vec(inner), parent_decl_context_id }
            }
            ASTNode::CallExpr { id, inner, value_category, type_, .. } => {
                ASTNodeCleanedUp::CallExpr { id, inner: Self::from_unclean_vec(inner), value_category, type_ }
            }
            ASTNode::AllocSizeAttr { implicit, inherited, id, .. } => {
                ASTNodeCleanedUp::AllocSizeAttr { implicit, inherited, id }
            }
            ASTNode::AttributedStmt { id, inner, .. } => {
                ASTNodeCleanedUp::AttributedStmt { id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::FinalAttr { id, .. } => {
                ASTNodeCleanedUp::FinalAttr { id }
            }
            ASTNode::AlignedAttr { id, inner, .. } => {
                ASTNodeCleanedUp::AlignedAttr { id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::CXXFunctionalCastExpr { type_, inner, cast_kind, value_category, id, .. } => {
                ASTNodeCleanedUp::CXXFunctionalCastExpr { type_, inner: Self::from_unclean_vec(inner), cast_kind, value_category, id }
            }
            ASTNode::InitListExpr { value_category, inner, type_, id, array_filler, field, .. } => {
                ASTNodeCleanedUp::InitListExpr { value_category, inner: Self::from_unclean_vec_option(inner), type_, id, array_filler: Self::from_unclean_vec_option(array_filler), field: Self::from_unclean_option_box(field) }
            }
            ASTNode::TemplateTemplateParmDecl { index, inner, depth, id, is_implicit, name, .. } => {
                ASTNodeCleanedUp::TemplateTemplateParmDecl { index, inner: Self::from_unclean_vec(inner), depth, id, is_implicit, name }
            }
            ASTNode::CXXBindTemporaryExpr { value_category, inner, id, temp, type_, dtor, .. } => {
                ASTNodeCleanedUp::CXXBindTemporaryExpr { value_category, inner: Self::from_unclean_vec(inner), id, temp, type_, dtor: Box::new(Self::from_unclean(*dtor)) }
            }
            ASTNode::IfStmt { inner, id, has_var, has_else, is_constexpr, .. } => {
                ASTNodeCleanedUp::IfStmt { inner: Self::from_unclean_vec(inner), id, has_var, has_else, is_constexpr }
            }
            ASTNode::ReturnsNonNullAttr { inherited, id, implicit, .. } => {
                ASTNodeCleanedUp::ReturnsNonNullAttr { inherited, id, implicit }
            }
            ASTNode::ExprWithCleanups { cleanups_have_side_effects, type_, inner, id, value_category, .. } => {
                ASTNodeCleanedUp::ExprWithCleanups { cleanups_have_side_effects, type_, inner: Self::from_unclean_vec(inner), id, value_category }
            }
            ASTNode::PointerType { type_, id, is_dependent, is_instantiation_dependent, inner, .. } => {
                ASTNodeCleanedUp::PointerType { type_, id, is_dependent, is_instantiation_dependent, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::CXXCatchStmt { id, inner, .. } => {
                ASTNodeCleanedUp::CXXCatchStmt { id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::AllocAlignAttr { implicit, inherited, id, .. } => {
                ASTNodeCleanedUp::AllocAlignAttr { implicit, inherited, id }
            }
            ASTNode::ParenExpr { value_category, id, type_, inner, .. } => {
                ASTNodeCleanedUp::ParenExpr { value_category, id, type_, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::UnresolvedMemberExpr { value_category, id, type_, .. } => {
                ASTNodeCleanedUp::UnresolvedMemberExpr { value_category, id, type_ }
            }
            ASTNode::CXXRecordDecl { complete_definition, is_referenced, is_implicit, id, previous_decl, tag_used, definition_data, name, bases, parent_decl_context_id, inner, .. } => {
                ASTNodeCleanedUp::CXXRecordDecl { complete_definition, is_referenced, is_implicit, id, previous_decl, tag_used, definition_data, name, bases, parent_decl_context_id, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::TypeTraitExpr { type_, id, value_category, .. } => {
                ASTNodeCleanedUp::TypeTraitExpr { type_, id, value_category }
            }
            ASTNode::DependentNameType { is_dependent, type_, is_instantiation_dependent, id, .. } => {
                ASTNodeCleanedUp::DependentNameType { is_dependent, type_, is_instantiation_dependent, id }
            }
            ASTNode::TypedefType { type_, inner, decl, id, is_dependent, is_instantiation_dependent, .. } => {
                ASTNodeCleanedUp::TypedefType { type_, inner: Self::from_unclean_vec(inner), decl: Box::new(Self::from_unclean(*decl)), id, is_dependent, is_instantiation_dependent }
            }
            ASTNode::LifetimeBoundAttr { id, implicit, .. } => {
                ASTNodeCleanedUp::LifetimeBoundAttr { id, implicit }
            }
            ASTNode::CXXTemporaryObjectExpr { inner, construction_kind, list, ctor_type, type_, value_category, id, had_multiple_candidates, zeroing, .. } => {
                ASTNodeCleanedUp::CXXTemporaryObjectExpr { inner: Self::from_unclean_vec_option(inner), construction_kind, list, ctor_type, type_, value_category, id, had_multiple_candidates, zeroing }
            }
            ASTNode::CXXCtorInitializer { any_init, inner, base_init, .. } => {
                ASTNodeCleanedUp::CXXCtorInitializer { any_init: Self::from_unclean_option_box(any_init), inner: Self::from_unclean_vec(inner), base_init }
            }
            ASTNode::CXXConstructorDecl { variadic, id, constexpr, previous_decl, is_implicit, inner, explicitly_deleted, mangled_name, is_referenced, is_used, parent_decl_context_id, inline, type_, explicitly_defaulted, name, .. } => {
                ASTNodeCleanedUp::CXXConstructorDecl { variadic, id, constexpr, previous_decl, is_implicit, inner: Self::from_unclean_vec_option(inner), explicitly_deleted, mangled_name, is_referenced, is_used, parent_decl_context_id, inline, type_, explicitly_defaulted, name }
            }
            ASTNode::WarnUnusedResultAttr { inherited, id, .. } => {
                ASTNodeCleanedUp::WarnUnusedResultAttr { inherited, id }
            }
            ASTNode::CXXOperatorCallExpr { id, value_category, inner, type_, .. } => {
                ASTNodeCleanedUp::CXXOperatorCallExpr { id, value_category, inner: Self::from_unclean_vec(inner), type_ }
            }
            ASTNode::SubstNonTypeTemplateParmExpr { inner, id, type_, value_category, .. } => {
                ASTNodeCleanedUp::SubstNonTypeTemplateParmExpr { inner: Self::from_unclean_vec(inner), id, type_, value_category }
            }
            ASTNode::NamespaceDecl { is_nested, id, inner, previous_decl, is_inline, original_namespace, name, .. } => {
                ASTNodeCleanedUp::NamespaceDecl { is_nested, id, inner: Self::from_unclean_vec_option(inner), previous_decl, is_inline, original_namespace: Self::from_unclean_option_box(original_namespace), name }
            }
            ASTNode::FriendDecl { id, inner, type_, .. } => {
                ASTNodeCleanedUp::FriendDecl { id, inner : Self::from_unclean_vec_option(inner), type_ }
            }
            ASTNode::TemplateTypeParmType { depth, index, decl, id, type_, is_dependent, contains_unexpanded_pack, is_pack, is_instantiation_dependent, .. } => {
                ASTNodeCleanedUp::TemplateTypeParmType { depth, index, decl: Box::new(Self::from_unclean(*decl)), id, type_, is_dependent, contains_unexpanded_pack, is_pack, is_instantiation_dependent }
            }
            ASTNode::ForStmt { inner, id, .. } => {
                ASTNodeCleanedUp::ForStmt { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::CXXNoexceptExpr { type_, id, value_category, inner, .. } => {
                ASTNodeCleanedUp::CXXNoexceptExpr { type_, id, value_category, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::TypeAliasDecl { id, type_, is_referenced, name, inner, .. } => {
                ASTNodeCleanedUp::TypeAliasDecl { id, type_, is_referenced, name, inner: Self::from_unclean_vec_option(inner) }
            }
            ASTNode::ImplicitCastExpr { id, inner, conversion_func, cast_kind, value_category, is_part_of_explicit_cast, type_, .. } => {
                ASTNodeCleanedUp::ImplicitCastExpr { id, inner: Self::from_unclean_vec(inner), conversion_func: Self::from_unclean_option_box(conversion_func), cast_kind, value_category, is_part_of_explicit_cast, type_ }
            }
            ASTNode::StaticAssertDecl { inner, id, .. } => {
                ASTNodeCleanedUp::StaticAssertDecl { inner: Self::from_unclean_vec(inner), id }
            }
            ASTNode::CStyleCastExpr { inner, type_, cast_kind, id, value_category, .. } => {
                ASTNodeCleanedUp::CStyleCastExpr { inner: Self::from_unclean_vec(inner), type_, cast_kind, id, value_category }
            }
            ASTNode::IndirectFieldDecl { inner, id, is_implicit, name, .. } => {
                ASTNodeCleanedUp::IndirectFieldDecl { inner: Self::from_unclean_vec_option(inner), id, is_implicit, name }
            }
            ASTNode::EmptyDecl { id, .. } => {
                ASTNodeCleanedUp::EmptyDecl { id }
            }
            ASTNode::BuiltinTemplateDecl { id, is_implicit, name, inner, .. } => {
                ASTNodeCleanedUp::BuiltinTemplateDecl { id, is_implicit, name, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::ArrayInitIndexExpr { type_, id, value_category, .. } => {
                ASTNodeCleanedUp::ArrayInitIndexExpr { type_, id, value_category }
            }
            ASTNode::WhileStmt { id, inner, .. } => {
                ASTNodeCleanedUp::WhileStmt { id, inner: Self::from_unclean_vec(inner) }
            }
            ASTNode::AbiTagAttr { id, .. } => {
                ASTNodeCleanedUp::AbiTagAttr { id }
            }
            ASTNode::NoThrowAttr { implicit, id, .. } => {
                ASTNodeCleanedUp::NoThrowAttr { implicit, id }
            }
            ASTNode::UnaryOperator { type_, id, value_category, can_overflow, inner, is_postfix, opcode, .. } => {
                ASTNodeCleanedUp::UnaryOperator { type_, id, value_category, can_overflow, inner: Self::from_unclean_vec(inner), is_postfix, opcode }
            }
            ASTNode::MemberExpr { value_category, inner, name, referenced_member_decl, type_, non_odr_use_reason, id, is_arrow, .. } => {
                ASTNodeCleanedUp::MemberExpr { value_category, inner: Self::from_unclean_vec(inner), name, referenced_member_decl, type_, non_odr_use_reason, id, is_arrow }
            }
        }
    }
}

fn from_hex<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
}

pub fn to_hex<S>(
    id: &u64,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    let s = format!("{:#014x}", id);
    serializer.serialize_str(s.as_str())
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
