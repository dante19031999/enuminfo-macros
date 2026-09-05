

# enuminfo

A collection of procedural derive macros for Rust `enum` types providing boolean checkers, string conversions, name lookup, and casing transformations compatible with `serde`.

---

## Features

- **`#[derive(EnumIs)]`**: Generates boolean helper methods (`is_<variant_name>()`) for all enum variants.
- **`#[derive(EnumName)]`**: Generates `const` methods (`name()` and `raw_name()`) that return the string representation of an enum variant.
- **`#[derive(EnumFromName)]`**: Generates lookup constructors (`from_name()` and `from_raw_name()`) to instantiate unit enum variants from string names.
- **Serde-compatible Casing**: Supports container-level `rename_all` attributes using standard casing options (`snake_case`, `kebab-case`, `camelCase`, etc.).

---

## Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
enuminfo = "0.1" # Replace with your crate's version

```

---

## Usage & Examples

### 1. `EnumIs`

Generates `is_<variant_name>(&self) -> bool` methods in `snake_case` for every variant.

```rust
use enuminfo::EnumIs;

#[derive(EnumIs)]
enum UserRole {
    Admin,
    StandardUser,
    SuperAdmin,
}

fn main() {
    let role = UserRole::StandardUser;

    assert!(role.is_standard_user());
    assert!(!role.is_admin());
    assert!(!role.is_super_admin());
}

```

---

### 2. `EnumName`

Provides string inspection methods:

* `name(&self) -> &'static str`: Returns the formatted string name (considering `rename` and `rename_all`).
* `raw_name(&self) -> &'static str`: Returns the exact identifier name as written in Rust code.

```rust
use enuminfo::EnumName;

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

* `from_name(name: &str) -> Option<Self>`: Matches formatted names (respecting `rename` and `rename_all`).
* `from_raw_name(name: &str) -> Option<Self>`: Matches original identifier names.

> **Note:** `EnumFromName` currently supports unit enums (variants without data fields).

```rust
use enuminfo::EnumFromName;

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

    // Ignored variants will not match via `from_name`
    assert_eq!(Priority::from_name("critical"), None);
}

```

---

## Attribute Reference

| Attribute | Target | Description | Example |
| --- | --- | --- | --- |
| `#[enuminfo(rename_all = "...")]` | Enum | Applies a casing convention to all variants | `#[enuminfo(rename_all = "snake_case")]` |
| `#[enuminfo(rename = "...")]` | Variant | Overrides the formatted name for a specific variant | `#[enuminfo(rename = "custom_val")]` |
| `#[enuminfo(ignore_from_name)]` | Variant | Excludes a variant from `EnumFromName::from_name()` | `#[enuminfo(ignore_from_name)]` |

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

---

## License

Dual-licensed under [MIT](https://www.google.com/search?q=LICENSE-MIT) or [Apache 2.0](https://www.google.com/search?q=LICENSE-APACHE).

