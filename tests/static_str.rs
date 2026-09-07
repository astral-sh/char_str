use char_str::{CharStr, CharString};

const TEXT: &str = "a string longer than the inline limit";
static STRING: CharString = CharString::from_static_str(TEXT);
static FROZEN: CharStr = CharStr::from_static_str(TEXT);
const STRING_TEXT: Option<&str> = STRING.as_static_str();
const FROZEN_TEXT: Option<&str> = FROZEN.as_static_str();

#[test]
fn static_borrows_outlive_the_owner() {
    assert_eq!(STRING_TEXT, Some(TEXT));
    assert_eq!(FROZEN_TEXT, Some(TEXT));
    let string_text = CharString::from_static_str(TEXT).as_static_str();
    let frozen_text = CharStr::from_static_str(TEXT).as_static_str();
    assert_eq!(string_text, Some(TEXT));
    assert_eq!(frozen_text, Some(TEXT));
    assert_eq!(frozen_text.unwrap().as_ptr(), TEXT.as_ptr());
}

#[test]
fn owned_storage_has_no_static_borrow() {
    for text in ["", "short", TEXT] {
        assert_eq!(CharString::from(text).as_static_str(), None);
        assert_eq!(CharStr::from(text).as_static_str(), None);
        assert_eq!(CharStr::new_heap(text).as_static_str(), None);
    }
    assert_eq!(CharString::from_static_str("short").as_static_str(), None);
    assert_eq!(CharStr::from_static_str("short").as_static_str(), None);
}

#[test]
fn static_borrows_follow_length_and_storage_changes() {
    let mut string = CharString::from_static_str(TEXT);
    let original = string.clone();
    string.truncate(3);
    assert_eq!(string.as_static_str(), Some("a s"));
    assert_eq!(original.as_static_str(), Some(TEXT));

    let frozen = string.freeze();
    assert_eq!(frozen.as_static_str(), Some("a s"));
    let mut string = frozen.into_char_string();
    string.push('!');
    assert_eq!(string.as_static_str(), None);
    assert_eq!(string, "a s!");
    assert_eq!(original.as_static_str(), Some(TEXT));
}
