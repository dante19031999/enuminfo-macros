
use enuminfo_macros::EnumName; // Cambia enuminfo_macros por el nombre de tu crate de macros en Cargo.toml

// 1. Enum simple sin modificaciones de nombre
#[derive(EnumName)]
#[allow(dead_code)]
enum SimpleEnum {
    FirstVariant,
    SecondVariant,
}

// 2. Enum con rename_all y variantes mixtas (unitaria, tupla, struct)
#[derive(EnumName)]
#[enuminfo(rename_all = "snake_case")]
#[allow(dead_code)]
enum SnakeEnum {
    UnitVariant,
    TupleVariant(i32, String),
    StructVariant { id: u64, active: bool },
}

// 3. Enum con rename_all y sobrescritura individual mediante rename
#[derive(EnumName)]
#[enuminfo(rename_all = "kebab-case")]
#[allow(dead_code)]
enum CustomRenamedEnum {
    NormalVariant,
    #[enuminfo(rename = "OVERRIDDEN_NAME")]
    SpecialVariant(String),
}

#[test]
fn test_simple_enum_names() {
    let v1 = SimpleEnum::FirstVariant;
    let v2 = SimpleEnum::SecondVariant;

    assert_eq!(v1.name(), "FirstVariant");
    assert_eq!(v1.raw_name(), "FirstVariant");

    assert_eq!(v2.name(), "SecondVariant");
    assert_eq!(v2.raw_name(), "SecondVariant");
}

#[test]
fn test_rename_all_snake_case_and_variant_types() {
    let unit = SnakeEnum::UnitVariant;
    let tuple = SnakeEnum::TupleVariant(42, "hello".to_string());
    let struct_var = SnakeEnum::StructVariant { id: 1, active: true };

    // Validar name() transformado a snake_case
    assert_eq!(unit.name(), "unit_variant");
    assert_eq!(tuple.name(), "tuple_variant");
    assert_eq!(struct_var.name(), "struct_variant");

    // Validar raw_name() preservando el identificador original de Rust
    assert_eq!(unit.raw_name(), "UnitVariant");
    assert_eq!(tuple.raw_name(), "TupleVariant");
    assert_eq!(struct_var.raw_name(), "StructVariant");
}

#[test]
fn test_individual_rename_override() {
    let normal = CustomRenamedEnum::NormalVariant;
    let special = CustomRenamedEnum::SpecialVariant("data".to_string());

    // Normal sigue la regla rename_all = "kebab-case"
    assert_eq!(normal.name(), "normal-variant");
    assert_eq!(normal.raw_name(), "NormalVariant");

    // Special sobrescribe name() mediante el atributo #[enuminfo(rename = "...")]
    assert_eq!(special.name(), "OVERRIDDEN_NAME");
    assert_eq!(special.raw_name(), "SpecialVariant");
}