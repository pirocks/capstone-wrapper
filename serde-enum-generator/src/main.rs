use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

use heck::ToSnakeCase;
use serde_json::Value;

fn extract_all_objects_impl<'a>(object: &'a Value, out: &mut Vec<&'a Value>) {
    match object {
        Value::Null |
        Value::Bool(_) |
        Value::Number(_) |
        Value::String(_) => {}
        Value::Array(arr) => {
            for val in arr {
                extract_all_objects_impl(val, out)
            }
        }
        Value::Object(obj) => {
            out.push(object);
            for val in obj.values() {
                extract_all_objects_impl(val, out)
            }
        }
    }
}

fn extract_all_objects(object: &Value) -> Vec<&Value> {
    let mut res = vec![];
    extract_all_objects_impl(object, &mut res);
    res
}

fn unique_field_combinations<'a>(objects: &[&'a Value]) -> HashSet<BTreeSet<&'a str>> {
    let mut res = HashSet::new();
    for val in objects {
        let mut fields = BTreeSet::new();
        for field in val.as_object().unwrap().keys() {
            fields.insert(field.as_str());
        };
        res.insert(fields);
    }
    res
}

fn classify_objects_by_discriminant<'a>(objects: &[&'a Value], discriminant: impl Into<String>) -> HashMap<&'a str, Vec<&'a Value>> {
    let discriminant = discriminant.into();
    let mut res: HashMap<&str, Vec<&Value>> = HashMap::new();
    for object in objects {
        let object_fields = object.as_object().unwrap();
        let discriminant_value = match object_fields.get(&discriminant) {
            Some(x) => x,
            None => continue,
        };
        res.entry(discriminant_value.as_str().unwrap()).or_default().push(object);
    }
    res
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Bool,
    Obj,
    Array,
}

#[derive(Debug)]
struct Field {
    field_type: Option<FieldType>,
    optional: bool,
    name: String,
}

struct FieldInternal {
    field_type: Option<FieldType>,
    count: usize,
}

fn fields_and_types(objects: &[&Value], discriminant: impl Into<String>) -> Vec<Field> {
    let discriminant = discriminant.into();
    let mut fields = HashMap::new();
    for object in objects {
        'inner: for (field_name, value) in object.as_object().unwrap() {
            if field_name == &discriminant {
                continue 'inner;
            }
            let field_type = match value {
                Value::Null => None,
                Value::Bool(_) => Some(FieldType::Bool),
                Value::Number(_) => Some(FieldType::Number),
                Value::String(_) => Some(FieldType::String),
                Value::Object(_) => Some(FieldType::Obj),
                Value::Array(_) => Some(FieldType::Array),
            };
            let field = fields.entry(field_name.to_string()).or_insert(FieldInternal {
                field_type,
                count: 0,
            });
            field.count += 1;
            if field.field_type != field_type {
                field.field_type = None;
            }
        }
    }
    let num_objects = objects.len();
    let mut fields_final = vec![];
    for (field_name, FieldInternal { field_type, count }) in fields {
        let optional = count != num_objects;
        fields_final.push(Field {
            field_type,
            optional,
            name: field_name,
        })
    }
    fields_final
}

fn fields_and_types_from_classified_objects<'a>(classified: &HashMap<&'a str, Vec<&'a Value>>, discriminant: impl Into<String>) -> HashMap<&'a str, Vec<Field>> {
    let mut res = HashMap::new();
    let discriminant = discriminant.into();
    for (name, values) in classified {
        res.insert(*name, fields_and_types(values.as_slice(), &discriminant));
    }
    res
}

fn should_rename(str: impl AsRef<str>) -> Option<String> {
    let str = str.as_ref();
    if str == "type" {
        return Some("type_".to_string());
    }
    if str.to_snake_case().as_str() == str {
        return None;
    }
    return Some(str.to_snake_case());
}

fn type_override(str: impl AsRef<str>) -> Option<String> {
    let str = str.as_ref();
    match str {
        "loc" => Some("Loc".to_string()),
        "range" => Some("ASTRange".to_string()),
        "explicitlyDefaulted" => Some("ExplicitlyDefaulted".to_string()),
        "definitionData" => Some("DefinitionData".to_string()),
        "tagUsed" => Some("TagUsed".to_string()),
        "bases" => Some("Vec<Base>".to_string()),
        "scopedEnumTag" => Some("ScopedEnumTag".to_string()),
        "nonOdrUseReason" => Some("NonOdrUseReason".to_string()),
        "castKind" => Some("CastKind".to_string()),
        "type" => Some("ASTType".to_string()),
        _ => None
    }
}

fn single_variant_serde_decl(name: &str, fields: &[Field]) -> String {
    let mut fields_strs = vec![];
    for field in fields {
        let field_name = field.name.to_string();
        let field_type_str = match field.field_type {
            None => {
                "!"
            }
            Some(FieldType::String) => {
                "String"
            }
            Some(FieldType::Obj) => {
                "Box<Self>"
            }
            Some(FieldType::Array) => {
                "Vec<Self>"
            }
            Some(FieldType::Bool) => {
                "bool"
            }
            Some(FieldType::Number) => {
                "i64"
            }
        };
        let field_type_str = match type_override(&field_name) {
            None => field_type_str.to_string(),
            Some(x) => x
        };
        let field_type_str = if field.optional {
            format!("Option<{field_type_str}>")
        } else {
            field_type_str.to_string()
        };
        match should_rename(&field_name) {
            None => {
                fields_strs.push(format!("{field_name} : {field_type_str}"));
            }
            Some(renamed) => {
                fields_strs.push(format!("\
                #[serde(rename = \"{field_name}\")]
                {renamed} : {field_type_str}"));
            }
        }
    }
    format!("{name} {{
{}
    }}", fields_strs.join(","))
}

fn to_serde_decl(fields_and_types: &HashMap<&str, Vec<Field>>) -> String {
    let variants: Vec<String> = fields_and_types.iter().map(|(name, fields)| single_variant_serde_decl(name, fields.as_slice())).collect();
    variants.join(",\n")
}

fn main() -> anyhow::Result<()> {
    let discriminant_name = "kind";

    let json_file: Value = serde_json::from_slice(fs::read("/home/francis/CLionProjects/capstone-wrapper/remill-semantics-parser/data/Instructions.json")?.as_slice())?;
    let all_objects = extract_all_objects(&json_file);
    // in this case we have a discriminant field on almost everything so just use that.
    //
    let classified = classify_objects_by_discriminant(all_objects.as_slice(), discriminant_name);
    let fields_and_types = fields_and_types_from_classified_objects(&classified, discriminant_name);
    println!("{}", to_serde_decl(&fields_and_types));

    Ok(())
}
