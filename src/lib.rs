//! A collection of procedural derive macros providing inspection, string conversion,
//! and construction utilities for Rust enums.
//!
//! # Overview
//!
//! This crate provides three derive macros:
//! - [`EnumIs`]: Generates boolean checker methods (`is_variant()`) for enum variants.
//! - [`EnumName`]: Generates methods (`name()` and `raw_name()`) returning variant names as string slices.
//! - [`EnumFromName`]: Generates lookup methods (`from_name()` and `from_raw_name()`) to construct unit enum variants from string names.
//!
//! # Case Conversions (`rename_all`)
//!
//! The `#[enuminfo(rename_all = "...")]` container attribute modifies how variant names are matched or returned.
//! The supported casing conventions match common values used by **`serde`**:
//!
//! | Option | Conversion Type | Example (`MyVariant`) |
//! |---|---|---|
//! | `"lowercase"` | All lower case | `"myvariant"` |
//! | `"UPPERCASE"` | All upper case | `"MYVARIANT"` |
//! | `"camelCase"` / `"lowerCamelCase"` | Lower camel case | `"myVariant"` |
//! | `"snake_case"` | Snake case | `"my_variant"` |
//! | `"SCREAMING-SNAKE-CASE"` | Screaming snake case | `"MY_SNAKE_CASE"` |
//! | `"kebab-case"` | Kebab case | `"my-variant"` |
//! | `"SCREAMING-KEBAB-CASE"` | Screaming kebab case | `"MY-KEBAB-CASE"` |
//! | `"PascalCase"` / `"UpperCamelCase"` | Pascal / Upper camel case | `"MyVariant"` |
//!
//! > **Note:** Casing options and behavior are aligned with standard `serde` attribute names for consistency across serialization and enum utilities.

mod enum_from_name;
mod enum_is;
mod enum_name;
mod enum_variants;
mod rename_case;

extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput};

/// Derives helper methods to check if an enum instance matches a specific variant.
///
/// For every variant `VariantName`, this macro implements a `const` method named `is_variant_name(&self) -> bool` in `snake_case`.
///
/// # Limitations
///
/// Works only on `enum` types.
///
/// # Example
///
/// ```rust
/// use enuminfo_macros::EnumIs;
///
/// #[derive(EnumIs)]
/// enum UserRole {
///     Admin,
///     StandardUser,
/// }
///
/// let role = UserRole::Admin;
/// assert!(role.is_admin());
/// assert!(!role.is_standard_user());
/// ```
#[proc_macro_derive(EnumIs)]
pub fn enum_is(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_is::implement_enum_is(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives methods to return the string representation of an enum variant.
///
/// This macro implements two `const` methods on the enum:
/// - `name(&self) -> &'static str`: Returns the (potentially renamed) variant name.
/// - `raw_name(&self) -> &'static str`: Returns the original identifier name as defined in code.
///
/// # Attributes
///
/// - `#[enuminfo(rename_all = "...")]`: Transforms variant output strings using `serde`-compatible casing names (e.g. `"snake_case"`, `"kebab-case"`).
/// - `#[enuminfo(rename = "custom_name")]`: Overrides the output string for a specific variant.
///
/// # Limitations
///
/// Works only on `enum` types.
///
/// # Example
///
/// ```rust
/// use enuminfo_macros::EnumName;
///
/// #[derive(EnumName)]
/// #[enuminfo(rename_all = "snake_case")]
/// enum Status {
///     Pending,
///     #[enuminfo(rename = "in_progress_custom")]
///     InProgress,
/// }
///
/// let status = Status::Pending;
/// assert_eq!(status.name(), "pending");
/// assert_eq!(status.raw_name(), "Pending");
///
/// let in_progress = Status::InProgress;
/// assert_eq!(in_progress.name(), "in_progress_custom");
/// assert_eq!(in_progress.raw_name(), "InProgress");
/// ```
#[proc_macro_derive(EnumName, attributes(enuminfo))]
pub fn enum_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_name::implement_enum_name(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives methods to construct unit enum variants from string names.
///
/// This macro implements two methods on the enum:
/// - `from_name(name: &str) -> Option<Self>`: Matches against the (potentially renamed) variant names.
/// - `from_raw_name(name: &str) -> Option<Self>`: Matches strictly against original variant identifier names.
///
/// # Attributes
///
/// - `#[enuminfo(rename_all = "...")]`: Sets variant lookup keys matching `serde`-compatible casing options.
/// - `#[enuminfo(rename = "custom_name")]`: Overrides the lookup key for `from_name()`.
/// - `#[enuminfo(ignore_from_name)]`: Excludes the variant from being constructed via `from_name()`.
///
/// # Limitations
///
/// This macro only supports **Unit Enums** (variants without fields).
///
/// # Example
///
/// ```rust
/// use enuminfo_macros::EnumFromName;
///
/// #[derive(Debug, PartialEq, EnumFromName)]
/// #[enuminfo(rename_all = "lowercase")]
/// enum Priority {
///     Low,
///     High,
///     #[enuminfo(ignore_from_name)]
///     Critical,
/// }
///
/// assert_eq!(Priority::from_name("low"), Some(Priority::Low));
/// assert_eq!(Priority::from_raw_name("Low"), Some(Priority::Low));
/// assert_eq!(Priority::from_name("critical"), None);
/// ```
#[proc_macro_derive(EnumFromName, attributes(enuminfo))]
pub fn enum_from_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_from_name::implement_enum_from_name(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives a static slice containing all variants of a unit enum.
///
/// This macro implements a public associated constant on the enum:
/// - `pub const VARIANTS: &'static [Self]`: Contains an array slice of all (non-ignored) unit variants.
///
/// # Attributes
///
/// - `#[enuminfo(ignore_variants)]`: Excludes the variant from the generated `VARIANTS` slice.
/// - `#[enuminfo(rename = "...")]` / `#[enuminfo(rename_all = "...")]`: Evaluates custom/cased names to prevent duplicate collisions in the static array, though the instances themselves remain untouched.
///
/// # Limitations
///
/// This macro only supports **Unit Enums** (variants without data fields, empty tuples `()`, or empty structs `{}`).
///
/// # Example
///
/// ```rust
/// use enuminfo_macros::EnumVariants;
///
/// #[derive(Debug, PartialEq, EnumVariants)]
/// enum Direction {
///     North,
///     South,
///     East,
///     West,
///     #[enuminfo(ignore_variants)]
///     Unknown,
/// }
///
/// assert_eq!(
///     Direction::VARIANTS,
///     &[Direction::North, Direction::South, Direction::East, Direction::West]
/// );
/// ```
#[proc_macro_derive(EnumVariants, attributes(enuminfo))]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_variants::implement_enum_variants(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

