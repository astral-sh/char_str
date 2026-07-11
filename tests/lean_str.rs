use lean_string::{LeanStr, LeanString};

const INLINE_LIMIT: usize = size_of::<LeanStr>();

#[test]
fn size() {
    assert_eq!(size_of::<LeanStr>(), 2 * size_of::<usize>());
    assert_eq!(size_of::<Option<LeanStr>>(), size_of::<LeanStr>());
}

#[test]
fn storage_kinds() {
    let inline = LeanStr::from("x".repeat(INLINE_LIMIT));
    let heap = LeanStr::from("x".repeat(INLINE_LIMIT + 1));
    const STATIC: LeanStr =
        LeanStr::from_static_str("a static string longer than the inline limit");

    assert!(!inline.is_heap_allocated());
    assert!(heap.is_heap_allocated());
    assert!(!STATIC.is_heap_allocated());
}

#[test]
fn clone_shares_heap_storage() {
    let one = LeanStr::from("a string longer than the inline limit");
    let two = one.clone();

    assert!(core::ptr::eq(one.as_ptr(), two.as_ptr()));
}

#[test]
fn shared_heap_conversions_copy_storage() {
    let mut string = LeanString::with_capacity(128);
    string.push_str("a string longer than the inline limit");
    let growable = string.clone();

    let frozen = string.freeze();
    let shared = frozen.clone();
    let mut thawed = frozen.into_lean_string();
    let thawed_ptr = thawed.as_ptr();
    thawed.push_str(" with more text");

    assert!(!core::ptr::eq(growable.as_ptr(), shared.as_ptr()));
    assert!(!core::ptr::eq(shared.as_ptr(), thawed_ptr));
    assert_eq!(growable, "a string longer than the inline limit");
    assert_eq!(shared, "a string longer than the inline limit");
    assert_eq!(thawed, "a string longer than the inline limit with more text");
}

#[test]
fn unique_heap_conversions_preserve_contents() {
    let text = "a string longer than the inline limit";
    let mut string = LeanString::with_capacity(128);
    string.push_str(text);

    let frozen = string.try_freeze().unwrap();
    let thawed = frozen.into_lean_string();

    assert_eq!(thawed, text);
    assert_eq!(thawed.capacity(), thawed.len());
}

#[test]
fn inline_and_static_conversions_preserve_storage_kind() {
    let inline = LeanString::from("short").freeze();
    assert!(!inline.is_heap_allocated());
    assert!(!inline.into_lean_string().is_heap_allocated());

    const TEXT: &str = "a static string longer than the inline limit";
    let string = LeanString::from_static_str(TEXT);
    let ptr = string.as_ptr();
    let frozen = string.freeze();

    assert!(!frozen.is_heap_allocated());
    assert!(core::ptr::eq(ptr, frozen.as_ptr()));

    let thawed = frozen.into_lean_string();
    assert!(!thawed.is_heap_allocated());
    assert!(core::ptr::eq(ptr, thawed.as_ptr()));
}

#[test]
fn heap_conversions_preserve_storage_for_short_contents() {
    let string = LeanString::with_capacity(INLINE_LIMIT + 1);
    assert!(string.is_heap_allocated());

    let frozen = string.freeze();
    assert!(frozen.is_heap_allocated());
    assert!(frozen.is_empty());

    let thawed = frozen.into_lean_string();
    assert!(thawed.is_heap_allocated());
    assert!(thawed.is_empty());
    assert_eq!(thawed.capacity(), 0);
}

#[test]
fn clear_unique_thawed_string_retains_growable_storage() {
    let mut thawed =
        LeanStr::from("a frozen string longer than the inline limit").into_lean_string();
    let capacity = thawed.capacity();

    thawed.clear();

    assert!(thawed.is_empty());
    assert_eq!(thawed.capacity(), capacity);
    assert!(thawed.is_heap_allocated());
}

#[test]
fn clear_shared_thawed_string_preserves_frozen_clone() {
    let frozen = LeanStr::from("a frozen string longer than the inline limit");
    let shared = frozen.clone();
    let mut thawed = frozen.into_lean_string();
    let capacity = thawed.capacity();

    thawed.clear();

    assert!(thawed.is_empty());
    assert_eq!(thawed.capacity(), capacity);
    assert!(thawed.is_heap_allocated());
    assert_eq!(shared, "a frozen string longer than the inline limit");
    assert!(shared.is_heap_allocated());
}

#[test]
fn shrink_to_fit_keeps_growable_heap_storage() {
    let text = "a string longer than the inline limit";
    let mut string = LeanString::with_capacity(128);
    string.push_str(text);

    string.shrink_to_fit();
    assert_eq!(string.capacity(), text.len());

    string.clear();
    assert!(string.is_empty());
    assert_eq!(string.capacity(), text.len());
    assert!(string.is_heap_allocated());
}

#[test]
fn collect_freezes_builder() {
    let frozen: LeanStr = "a string longer than the inline limit".chars().collect();

    assert_eq!(frozen, "a string longer than the inline limit");
    assert!(frozen.is_heap_allocated());
}
