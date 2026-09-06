# enuminfo-macros

A collection of procedural derive macros providing inspection, string conversion, lookup, and variant enumeration utilities for Rust enums.

[![Crates.io](https://img.shields.io/crates/v/enuminfo_macros.svg)](https://crates.io/crates/enuminfo_macros)
[![Documentation](https://docs.rs/enuminfo_macros/badge.svg)](https://docs.rs/enuminfo_macros)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)]()

---

## Features

- **`#[derive(EnumIs)]`**: Generates boolean helper methods (`is_<variant>()`) for all enum variants (supports unit, tuple, and struct variants).
- **`#[derive(EnumName)]`**: Generates `const` methods (`name()` and `raw_name()`) returning variant names as string slices.
- **`#[derive(EnumFromName)]`**: Generates lookup constructors (`from_name()` and `from_raw_name()`) to instantiate unit enum variants from string names, with optional `FromStr` support.
- **`#[derive(EnumVariants)]`**: Generates an associated constant array slice (`VARIANTS`) and helper methods (`variants()` and `variant_count()`) containing all unit variants.
- **Serde-compatible Casing**: Supports container-level `rename_all` attributes using standard casing options (`snake_case`, `kebab-case`, `camelCase`, etc.).
- **Compile-Time Safety**: Detects and rejects duplicate variant names caused by renaming or casing transformations at compile time.

---

## Installation

Add `enuminfo_macros` to your `Cargo.toml`:

```toml
[dependencies]
enuminfo_macros = "0.1"
```

*(If you are using the companion facade crate [`enuminfo`](https://crates.io/crates/enuminfo), these derive macros are re-exported automatically.)*

---

## Usage & Examples

### 1. `EnumIs`

Generates `pub const fn is_<variant>(&self) -> bool` methods in `snake_case` for every variant. Supports unit, tuple, and struct variants.

```rust
use enuminfo_macros::EnumIs;

#[derive(EnumIs)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    assert!(msg.is_move());
    assert!(!msg.is_quit());
    assert!(!msg.is_write());
}
```

---

### 2. `EnumName`

Provides `const` string inspection methods:

- `name(&self) -> &'static str`: Returns the formatted variant name (respecting `rename_all` and `rename`).
- `raw_name(&self) -> &'static str`: Returns the exact Rust identifier name as written in code.

Supports all variant types (unit, tuple, and struct variants) and validates at compile time that variant names are unique.

```rust
use enuminfo_macros::EnumName;

#[derive(EnumName)]
#[enuminfo(rename_all = "snake_case")]
enum Status {
    Pending,
    #[enuminfo(rename = "custom_in_progress")]
    InProgress,
    Completed,
}

fn main() {
    let status = Status::InProgress;

    // Converted name (respects variant rename override)
    assert_eq!(status.name(), "custom_in_progress");

    // Original identifier name
    assert_eq!(status.raw_name(), "InProgress");
}

```

---

### 3. `EnumFromName`

Constructs unit enum variants from string names:

- `from_name(name: &str) -> Option<Self>`: Matches formatted names (respecting `rename` and `rename_all`).
- `from_raw_name(name: &str) -> Option<Self>`: Matches original Rust identifier names.
- Also implements `std::str::FromStr` when the `from-str` feature is enabled (enabled by default).

> **Note:** `EnumFromName` supports unit enums (variants without fields, empty tuple `()`, or empty struct `{}`).

```rust
use enuminfo_macros::EnumFromName;

#[derive(Debug, PartialEq, EnumFromName)]
#[enuminfo(rename_all = "kebab-case")]
enum Priority {
    Low,
    MediumPriority,
    #[enuminfo(ignore_from_name)]
    Critical,
}

fn main() {
    // Lookup by formatted name
    assert_eq!(Priority::from_name("medium-priority"), Some(Priority::MediumPriority));

    // Lookup by raw identifier name
    assert_eq!(Priority::from_raw_name("MediumPriority"), Some(Priority::MediumPriority));

    // Ignored variants will not match via from_name or from_raw_name
    assert_eq!(Priority::from_name("critical"), None);
    assert_eq!(Priority::from_raw_name("Critical"), None);
}

```

---

### 4. `EnumVariants`

Generates an associated constant array slice and enumeration helper methods containing all unit variants:

- `pub const VARIANTS: &'static [Self]`: Associated static slice of all (non-ignored) unit variant instances.
- `pub const fn variants() -> &'static [Self]`: Returns `Self::VARIANTS`.
- `pub const fn variant_count() -> usize`: Returns the total number of variants (`Self::VARIANTS.len()`).

> **Note:** `EnumVariants` supports unit enums (variants without fields, empty tuple `()`, or empty struct `{}`).

```rust
use enuminfo_macros::EnumVariants;

#[derive(Debug, PartialEq, EnumVariants)]
enum Direction {
    North,
    South,
    East,
    West,
    #[enuminfo(ignore_variants)]
    Unknown,
}

fn main() {
    // Retrieve slice of all non-ignored variants
    assert_eq!(
        Direction::variants(),
        &[Direction::North, Direction::South, Direction::East, Direction::West]
    );

    // Access via the associated constant
    assert_eq!(Direction::VARIANTS.len(), 4);

    // Get the total variant count
    assert_eq!(Direction::variant_count(), 4);
}
```

---

## Attribute Reference

| Attribute | Target | Supported Macros | Description | Example |
| --- | --- | --- | --- | --- |
| `#[enuminfo(rename_all = "...")]` | Enum | `EnumName`, `EnumFromName`, `EnumVariants` | Applies a casing convention to all variant names | `#[enuminfo(rename_all = "snake_case")]` |
| `#[enuminfo(rename = "...")]` | Variant | `EnumName`, `EnumFromName`, `EnumVariants` | Overrides the formatted name for a specific variant | `#[enuminfo(rename = "custom_val")]` |
| `#[enuminfo(ignore_from_name)]` | Variant | `EnumFromName` | Excludes a variant from `from_name()`, `from_raw_name()`, and `FromStr` | `#[enuminfo(ignore_from_name)]` |
| `#[enuminfo(ignore_variants)]` | Variant | `EnumVariants` | Excludes a variant from `VARIANTS`, `variants()`, and `variant_count()` | `#[enuminfo(ignore_variants)]` |

> **Note:** `rename` and `rename_all` are accepted by `EnumVariants` for attribute compatibility when used alongside `EnumName` or `EnumFromName`, but they do not alter the enum instances in `VARIANTS`.

---

## Casing Conventions (`rename_all`)

The `rename_all` attribute supports standard `serde`-compatible casing strings:

| Value | Transformation | Input `MyVariant` |
| --- | --- | --- |
| `"lowercase"` | Lowercase | `"myvariant"` |
| `"UPPERCASE"` | Uppercase | `"MYVARIANT"` |
| `"camelCase"` / `"lowerCamelCase"` | Lower camel case | `"myVariant"` |
| `"snake_case"` | Snake case | `"my_variant"` |
| `"SCREAMING-SNAKE-CASE"` | Screaming snake case | `"MY_SNAKE_CASE"` |
| `"kebab-case"` | Kebab case | `"my-variant"` |
| `"SCREAMING-KEBAB-CASE"` | Screaming kebab case | `"MY-KEBAB-CASE"` |
| `"PascalCase"` / `"UpperCamelCase"` | Upper camel case | `"MyVariant"` |

If an unrecognized casing string is provided, variant names remain unmodified.

---

## Feature Flags

| Feature | Default | Description |
| --- | --- | --- |
| `from-str` | **Yes** | Implements `std::str::FromStr` for enums deriving `EnumFromName`. |
| `impl-enuminfo` | No | Implements companion traits from the [`enuminfo`](https://crates.io/crates/enuminfo) crate (`EnumIs`, `EnumName`, `EnumFromName`, `EnumVariants`). |
| `skip-inherent` | No | Skips generating inherent `impl` blocks; useful when only trait implementations are desired. |

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Acknowledgements: https://dante19031999.github.io/enuminfo-macros/ACKNOWLEDGEMENTS.html

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.