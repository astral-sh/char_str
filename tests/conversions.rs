use char_str::{CharStr, CharString};

#[test]
fn collect_owned_strings() {
    for parts in
        [vec![], vec![""], vec!["short"], vec!["longer than the inline limit", "🦀", "end"]]
    {
        let expected = parts.concat();
        assert_eq!(parts.iter().copied().map(CharStr::from).collect::<CharString>(), expected);
        assert_eq!(parts.iter().copied().map(CharString::from).collect::<CharStr>(), expected);
        assert_eq!(parts.iter().copied().map(CharStr::from).collect::<CharStr>(), expected);
        assert_eq!(parts.iter().copied().map(CharStr::from).collect::<String>(), expected);
        assert_eq!(parts.iter().copied().map(CharString::from).collect::<String>(), expected);
    }
}

#[test]
fn collecting_one_immutable_string_preserves_its_allocation() {
    let value = CharStr::new_heap("short");
    let shared = value.clone();
    let collected = [value].into_iter().collect::<CharStr>();
    assert_eq!(collected.as_ptr(), shared.as_ptr());
    assert_eq!(collected, shared);
}

#[test]
fn extend_with_immutable_strings() {
    let parts = [CharStr::from("🦀"), CharStr::from("longer than the inline limit")];
    let mut string = CharString::from("prefix");
    let mut standard = String::from("prefix");
    string.extend(parts.clone());
    standard.extend(parts);
    assert_eq!(string, "prefix🦀longer than the inline limit");
    assert_eq!(standard, string);
}

#[cfg(feature = "std")]
#[test]
fn strings_can_be_passed_to_path_apis() {
    use std::path::Path;
    fn as_path(value: &impl AsRef<Path>) -> &Path {
        value.as_ref()
    }
    for text in ["", "src/lib.rs", "directory/longer_than_the_inline_limit/🦀.rs"] {
        assert_eq!(as_path(&CharString::from(text)), Path::new(text));
        assert_eq!(as_path(&CharStr::from(text)), Path::new(text));
    }
}
