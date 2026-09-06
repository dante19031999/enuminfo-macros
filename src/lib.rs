//! A collection of procedural derive macros providing inspection, string conversion,
//! lookup, and variant enumeration utilities for Rust enums.
//!
//! # Overview
//!
//! This crate provides four derive macros:
//! - [`EnumIs`]: Generates boolean checker methods (`is_<variant>()`) for enum variants.
//! - [`EnumName`]: Generates `const` methods (`name()` and `raw_name()`) returning variant names as string slices.
//! - [`EnumFromName`]: Generates lookup constructors (`from_name()` and `from_raw_name()`) to instantiate unit enum variants from string names (with optional [`FromStr`](core::str::FromStr) support).
//! - [`EnumVariants`]: Generates an associated constant array slice (`VARIANTS`) and helper methods (`variants()` and `variant_count()`) containing all unit variants.
//!
//! # Attributes Reference
//!
//! The following attributes can be placed on enums or their variants under the `#[enuminfo(...)]` helper attribute:
//!
//! | Attribute | Target | Supported Macros | Description |
//! |---|---|---|---|
//! | `#[enuminfo(rename_all = "...")]` | Enum | [`EnumName`], [`EnumFromName`], [`EnumVariants`] | Applies a casing convention to variant names (e.g. `"snake_case"`). |
//! | `#[enuminfo(rename = "...")]` | Variant | [`EnumName`], [`EnumFromName`], [`EnumVariants`] | Overrides the formatted name for an individual variant. |
//! | `#[enuminfo(ignore_from_name)]` | Variant | [`EnumFromName`] | Excludes the variant from being constructed by `from_name()`, `from_raw_name()`, or `FromStr`. |
//! | `#[enuminfo(ignore_variant)]` | Variant | [`EnumVariants`] | Excludes the variant from the generated `VARIANTS` slice and `variant_count()`. |
//!
//! > **Note:** `#[enuminfo(rename = "...")]` and `#[enuminfo(rename_all = "...")]` are accepted by [`EnumVariants`] for attribute compatibility when used alongside [`EnumName`] or [`EnumFromName`], but they do not affect variant instances in `VARIANTS`.
//!
//! # Case Conversions (`rename_all`)
//!
//! The `#[enuminfo(rename_all = "...")]` container attribute modifies how variant names are matched or returned.
//! Supported casing conventions match the standard conventions used by **`serde`**:
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
//! If an unrecognized casing string is provided, variant names remain unmodified.
//!
//! # Feature Flags
//!
//! This crate provides several Cargo feature flags:
//!
//! - **`from-str`** *(enabled by default)*: Generates an implementation of [`core::str::FromStr`] for enums deriving [`EnumFromName`].
//! - **`impl-enuminfo`**: Implements companion traits from the [`enuminfo`](https://crates.io/crates/enuminfo) crate (`EnumIs`, `EnumName`, `EnumFromName`, `EnumVariants`).
//! - **`skip-inherent`**: Skips generating inherent `impl` blocks on the enum, generating only trait implementations when combined with `impl-enuminfo`.

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
/// For every variant `VariantName`, this macro implements a `const` method named
/// `is_<variant_name>(&self) -> bool` using `snake_case` naming.
///
/// # Compatibility & Limitations
///
/// - Works on `enum` types only.
/// - Supports **all** enum variant types: unit variants (e.g. `Quit`), tuple variants
///   (e.g. `Write(String)`), and struct variants (e.g. `Move { x: i32, y: i32 }`).
///
/// # Feature Flags
///
/// If the `impl-enuminfo` feature is enabled, this macro additionally implements
/// the `enuminfo::EnumIs` marker trait for the enum.
///
/// # Example
///
/// ```rust
/// # #[cfg(not(feature = "skip-inherent"))]
/// # {
/// use enuminfo_macros::EnumIs;
///
/// #[derive(EnumIs)]
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Write(String),
/// }
///
/// let msg = Message::Move { x: 10, y: 20 };
/// assert!(msg.is_move());
/// assert!(!msg.is_quit());
/// assert!(!msg.is_write());
/// # }
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
/// - `name(&self) -> &'static str`: Returns the formatted variant name, taking into account
///   any container `rename_all` casing transformation and variant-level `rename` overrides.
/// - `raw_name(&self) -> &'static str`: Returns the exact Rust identifier name as written in code.
///
/// # Attributes
///
/// - `#[enuminfo(rename_all = "...")]`: Container attribute that transforms output strings using
///   `serde`-compatible casing names (e.g. `"snake_case"`, `"kebab-case"`, `"camelCase"`).
/// - `#[enuminfo(rename = "custom_name")]`: Variant attribute that overrides the output string
///   for a specific variant.
///
/// # Compile-Time Checks
///
/// Validates at compile time that all variant names (after applying `rename_all` and `rename`)
/// are unique. If a duplicate name is detected, compilation fails with an error.
///
/// # Compatibility & Limitations
///
/// - Works on `enum` types only.
/// - Supports **all** enum variant types: unit variants, tuple variants, and struct variants.
///
/// # Feature Flags
///
/// If the `impl-enuminfo` feature is enabled, this macro additionally implements
/// the `enuminfo::EnumName` trait for the enum.
///
/// # Example
///
/// ```rust
/// # #[cfg(not(feature = "skip-inherent"))]
/// # {
/// use enuminfo_macros::EnumName;
///
/// #[derive(EnumName)]
/// #[enuminfo(rename_all = "snake_case")]
/// enum Status {
///     Pending,
///     #[enuminfo(rename = "in_progress_custom")]
///     InProgress,
///     Completed,
/// }
///
/// let status = Status::Pending;
/// assert_eq!(status.name(), "pending");
/// assert_eq!(status.raw_name(), "Pending");
///
/// let in_progress = Status::InProgress;
/// assert_eq!(in_progress.name(), "in_progress_custom");
/// assert_eq!(in_progress.raw_name(), "InProgress");
/// # }
/// ```
#[proc_macro_derive(EnumName, attributes(enuminfo))]
pub fn enum_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_name::implement_enum_name(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives constructor lookup methods to instantiate unit enum variants from string names.
///
/// This macro implements two lookup methods on the enum:
/// - `pub fn from_name(name: &str) -> Option<Self>`: Matches against the formatted variant name
///   (respecting `rename_all` casing rules and `rename` overrides).
/// - `pub fn from_raw_name(name: &str) -> Option<Self>`: Matches strictly against the original Rust variant identifier name.
///
/// Additionally, when the `from-str` feature is enabled (default), it generates an implementation
/// of [`core::str::FromStr`] that parses formatted variant names by delegating to `from_name()`.
///
/// # Attributes
///
/// - `#[enuminfo(rename_all = "...")]`: Container attribute setting variant lookup keys using
///   `serde`-compatible casing options.
/// - `#[enuminfo(rename = "custom_name")]`: Variant attribute overriding the lookup key for `from_name()`.
/// - `#[enuminfo(ignore_from_name)]`: Variant attribute excluding the variant from both `from_name()`
///   and `from_raw_name()` (and consequently from `FromStr`).
///
/// # Compile-Time Checks
///
/// Validates at compile time that all formatted variant lookup keys are unique. If duplicates
/// occur, compilation fails with an error.
///
/// # Limitations
///
/// This macro supports **Unit Enums** only. Variants must have no data fields:
/// - Unit variants (e.g. `Variant`)
/// - Empty tuple variants (e.g. `Variant()`)
/// - Empty struct variants (e.g. `Variant {}`)
///
/// Variants with fields will cause a compilation error.
///
/// # Feature Flags
///
/// - `from-str` *(default)*: Generates `impl std::str::FromStr for MyEnum`.
/// - `impl-enuminfo`: Implements the `enuminfo::EnumFromName` trait for the enum.
///
/// # Example
///
/// ```rust
/// # #[cfg(not(feature = "skip-inherent"))]
/// # {
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
/// // Formatted lookup
/// assert_eq!(Priority::from_name("low"), Some(Priority::Low));
///
/// // Raw identifier lookup
/// assert_eq!(Priority::from_raw_name("Low"), Some(Priority::Low));
///
/// // Ignored variants cannot be constructed by name
/// assert_eq!(Priority::from_name("critical"), None);
/// assert_eq!(Priority::from_raw_name("Critical"), None);
/// # }
/// ```
#[proc_macro_derive(EnumFromName, attributes(enuminfo))]
pub fn enum_from_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_from_name::implement_enum_from_name(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives a static slice and enumeration helpers containing all variants of a unit enum.
///
/// This macro implements the following items on the enum:
/// - `pub const VARIANTS: &'static [Self]`: An associated static slice containing all (non-ignored) unit variant instances.
/// - `pub const fn variants() -> &'static [Self]`: Returns `Self::VARIANTS`.
/// - `pub const fn variant_count() -> usize`: Returns the total number of variants in the static slice (`Self::VARIANTS.len()`).
///
/// # Attributes
///
/// - `#[enuminfo(ignore_variant)]`: Variant attribute that excludes the variant from the generated `VARIANTS` slice,
///   `variants()`, and `variant_count()`.
/// - `#[enuminfo(rename = "...")]` / `#[enuminfo(rename_all = "...")]`: Accepted on enum and variant levels
///   for attribute compatibility when derived together with [`EnumName`] or [`EnumFromName`], but has no effect
///   on the instances placed in `VARIANTS`.
///
/// # Limitations
///
/// This macro supports **Unit Enums** only. Variants must have no data fields:
/// - Unit variants (e.g. `Variant`)
/// - Empty tuple variants (e.g. `Variant()`)
/// - Empty struct variants (e.g. `Variant {}`)
///
/// Variants with fields will cause a compilation error.
///
/// # Feature Flags
///
/// If the `impl-enuminfo` feature is enabled, this macro additionally implements
/// the `enuminfo::EnumVariants` trait for the enum.
///
/// # Example
///
/// ```rust
/// # #[cfg(not(feature = "skip-inherent"))]
/// # {
/// use enuminfo_macros::EnumVariants;
///
/// #[derive(Debug, PartialEq, EnumVariants)]
/// enum Direction {
///     North,
///     South,
///     East,
///     West,
///     #[enuminfo(ignore_variant)]
///     Unknown,
/// }
///
/// // Retrieve slice of all variants
/// assert_eq!(
///     Direction::variants(),
///     &[Direction::North, Direction::South, Direction::East, Direction::West]
/// );
///
/// // Direct access to the associated constant
/// assert_eq!(Direction::VARIANTS.len(), 4);
///
/// // Get the total variant count
/// assert_eq!(Direction::variant_count(), 4);
/// # }
/// ```
#[proc_macro_derive(EnumVariants, attributes(enuminfo))]
pub fn enum_variants(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    enum_variants::implement_enum_variants(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

