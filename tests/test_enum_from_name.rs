#![cfg(any(not(feature = "skip-inherent"), feature = "impl-enuminfo"))]

use enuminfo_macros::EnumFromName;
#[cfg(all(feature = "skip-inherent", feature = "impl-enuminfo"))]
use enuminfo::*;
#[cfg(feature = "from-str")]
use std::str::FromStr;

// 1. Enum básico
#[derive(Debug, PartialEq, EnumFromName)]
#[allow(dead_code)]
enum SimpleEnum {
    FirstVariant,
    SecondVariant,
}

// 2. Enum con rename_all y rename individual
#[derive(Debug, PartialEq, EnumFromName)]
#[enuminfo(rename_all = "snake_case")]
#[allow(dead_code)]
enum CustomEnum {
    UserAccount,
    #[enuminfo(rename = "ADMIN_ROLE")]
    AdminRole,
}

// 3. Enum con ignore_from_name
#[derive(Debug, PartialEq, EnumFromName)]
#[enuminfo(rename_all = "kebab-case")]
#[allow(dead_code)]
enum IgnoredEnum {
    ActiveStatus,
    #[enuminfo(ignore_from_name)]
    IgnoredStatus,
}

#[derive(Debug, PartialEq, EnumFromName)]
#[allow(dead_code)]
enum TroubleSomeEnum {
    //FirstVariant(),
   // SecondVariant {},
}

#[test]
fn test_simple_enum_from_name() {
    assert_eq!(
        SimpleEnum::from_name("FirstVariant"),
        Some(SimpleEnum::FirstVariant)
    );
    assert_eq!(
        SimpleEnum::from_name("SecondVariant"),
        Some(SimpleEnum::SecondVariant)
    );
    assert_eq!(SimpleEnum::from_name("UnknownVariant"), None);

    assert_eq!(
        SimpleEnum::from_raw_name("FirstVariant"),
        Some(SimpleEnum::FirstVariant)
    );
    assert_eq!(
        SimpleEnum::from_raw_name("SecondVariant"),
        Some(SimpleEnum::SecondVariant)
    );
    assert_eq!(SimpleEnum::from_raw_name("UnknownVariant"), None);
}

#[test]
fn test_custom_rename_from_name() {
    // Probar conversión con rename_all = "snake_case"
    assert_eq!(
        CustomEnum::from_name("user_account"),
        Some(CustomEnum::UserAccount)
    );

    // Probar conversión con #[enuminfo(rename = "ADMIN_ROLE")]
    assert_eq!(
        CustomEnum::from_name("ADMIN_ROLE"),
        Some(CustomEnum::AdminRole)
    );

    // Validar que el nombre transformado viejo ya no coincide para AdminRole
    assert_eq!(CustomEnum::from_name("admin_role"), None);

    // Probar from_raw_name preservando identificadores de Rust
    assert_eq!(
        CustomEnum::from_raw_name("UserAccount"),
        Some(CustomEnum::UserAccount)
    );
    assert_eq!(
        CustomEnum::from_raw_name("AdminRole"),
        Some(CustomEnum::AdminRole)
    );
}

#[test]
fn test_ignored_variant_cannot_be_parsed() {
    // La variante normal se parsea sin problema
    assert_eq!(
        IgnoredEnum::from_name("active-status"),
        Some(IgnoredEnum::ActiveStatus)
    );
    assert_eq!(
        IgnoredEnum::from_raw_name("ActiveStatus"),
        Some(IgnoredEnum::ActiveStatus)
    );

    // La variante ignorada DEBE devolver None en ambos casos
    assert_eq!(IgnoredEnum::from_name("ignored-status"), None);
    assert_eq!(IgnoredEnum::from_raw_name("IgnoredStatus"), None);
}

#[cfg(feature = "from-str")]
#[test]
fn test_from_str() {
    // La variante normal se parsea sin problema
    assert_eq!(
        IgnoredEnum::from_str("active-status").unwrap(),
        IgnoredEnum::ActiveStatus
    );

    // La variante ignorada DEBE devolver None en ambos casos
    assert!(IgnoredEnum::from_str("ignored-status").is_err());
}


