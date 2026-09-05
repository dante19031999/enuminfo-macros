use enuminfo_macros::EnumVariants;

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
        Status::VARIANTS,
        &[Status::Pending, Status::Active, Status::Completed]
    );
}

#[test]
fn test_enum_variants_with_ignore() {
    #[derive(Debug, PartialEq, Eq, EnumVariants)]
    #[allow(dead_code)]
    enum Role {
        Admin,
        User,
        #[enuminfo(ignore_variants)]
        InternalSystem,
    }

    assert_eq!(Role::VARIANTS, &[Role::Admin, Role::User]);
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
        EmptyKinds::VARIANTS,
        &[EmptyKinds::Unit, EmptyKinds::Tuple(), EmptyKinds::Struct {}]
    );
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

    assert_eq!(Color::VARIANTS, &[Color::Red, Color::Green, Color::Blue]);
}