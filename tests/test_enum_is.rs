#![cfg(any(not(feature = "skip-inherent"), feature = "impl-enuminfo"))]
use enuminfo_macros::EnumIs;

#[derive(EnumIs)]
enum SimpleEnum {
    Case1,
    Case2,
}

#[test]
fn test_enum_is() {
    let case1 = SimpleEnum::Case1;
    let case2 = SimpleEnum::Case2;

    assert!(case1.is_case1());
    assert!(!case1.is_case2());

    assert!(!case2.is_case1());
    assert!(case2.is_case2());
}
