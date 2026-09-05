use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToUpperCamelCase,
};

pub enum RenameCase {
    None,
    LowerCase,
    UpperCase,
    LowerCamelCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
    UpperCamelCase,
}

impl RenameCase {
    pub(crate) fn from_str(value: &str) -> RenameCase {
        match value {
            "lowercase" => RenameCase::LowerCase,
            "UPPERCASE" => RenameCase::UpperCase,
            "camelCase" => RenameCase::LowerCamelCase,
            "lowerCamelCase" => RenameCase::LowerCamelCase,
            "snake_case" => RenameCase::SnakeCase,
            "SCREAMING-SNAKE-CASE" => RenameCase::ScreamingSnakeCase,
            "kebab-case" => RenameCase::KebabCase,
            "SCREAMING-KEBAB-CASE" => RenameCase::ScreamingKebabCase,
            "PascalCase" => RenameCase::UpperCamelCase,
            "UpperCamelCase" => RenameCase::UpperCamelCase,
            _ => RenameCase::None,
        }
    }

    pub fn apply(&self, value: &str) -> String {
        match self {
            RenameCase::None => value.to_string(),
            RenameCase::LowerCase => value.to_lowercase(),
            RenameCase::UpperCase => value.to_uppercase(),
            RenameCase::LowerCamelCase => value.to_lower_camel_case(),
            RenameCase::SnakeCase => value.to_snake_case(),
            RenameCase::ScreamingSnakeCase => value.to_shouty_snake_case(),
            RenameCase::KebabCase => value.to_kebab_case(),
            RenameCase::ScreamingKebabCase => value.to_shouty_kebab_case(),
            RenameCase::UpperCamelCase => value.to_upper_camel_case(),
        }
    }
}
