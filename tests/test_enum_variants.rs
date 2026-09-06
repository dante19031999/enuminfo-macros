#![cfg(any(not(feature = "skip-inherent"), feature = "impl-enuminfo"))]
use enuminfo_macros::EnumVariants;
#[cfg(all(feature = "skip-inherent", feature = "impl-enuminfo"))]
use enuminfo::*;

#[test]
fn test_basic_enum_variants() {
    #[derive(Debug, PartialEq, Eq, EnumVariants)]
    #[allow(dead_code)]
    enum Status {
        Pending,
        Active,
        Completed,
    }

    assert_eq!(
        Status::variants(),
        &[Status::Pending, Status::Active, Status::Completed]
    );
    assert_eq!(Status::variant_count(), 3);
}

#[test]
fn test_enum_variants_with_ignore() {
    #[derive(Debug, PartialEq, Eq, EnumVariants)]
    #[allow(dead_code)]
    enum Role {
        Admin,
        User,
        #[enuminfo(ignore_variant)]
        InternalSystem,
    }

    assert_eq!(Role::variants(), &[Role::Admin, Role::User]);
    assert_eq!(Role::variant_count(), 2);
}

#[test]
fn test_enum_variants_empty_tuple_and_struct_variants() {
    #[derive(Debug, PartialEq, Eq, EnumVariants)]
    enum EmptyKinds {
        Unit,
        Tuple(),
        Struct {},
    }

    assert_eq!(
        EmptyKinds::variants(),
        &[EmptyKinds::Unit, EmptyKinds::Tuple(), EmptyKinds::Struct {}]
    );
    assert_eq!(EmptyKinds::variant_count(), 3);
}

#[test]
fn test_enum_variants_with_rename_attributes() {
    // Aunque 'rename' y 'rename_all' cambian los nombres procesados en 'seen_names',
    // la constante VARIANTS debe seguir conteniendo la lista de instancias válidas.
    #[derive(Debug, PartialEq, Eq, EnumVariants)]
    #[enuminfo(rename_all = "snake_case")]
    enum Color {
        Red,
        #[enuminfo(rename = "custom_green")]
        Green,
        Blue,
    }

    assert_eq!(Color::variants(), &[Color::Red, Color::Green, Color::Blue]);
    assert_eq!(Color::variant_count(), 3);
}