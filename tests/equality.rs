use std::borrow::Cow;

use char_str::{CharStr, CharString};

fn check_equality(exact: &CharStr, growable: &CharString, text: &str) {
    let expected = exact.as_str() == text;
    assert_eq!(growable.as_str() == text, expected);

    let other_exact = CharStr::from(text);
    let other_growable = CharString::from(text);
    assert_eq!(exact == &other_exact, expected);
    assert_eq!(growable == &other_growable, expected);
    assert_eq!(exact == &other_growable, expected);
    assert_eq!(growable == &other_exact, expected);
    assert_eq!(&other_exact == exact, expected);
    assert_eq!(&other_growable == growable, expected);
    assert_eq!(&other_growable == exact, expected);
    assert_eq!(&other_exact == growable, expected);

    assert_eq!(exact.eq(text), expected);
    assert_eq!(growable.eq(text), expected);
    assert_eq!(text.eq(exact), expected);
    assert_eq!(text.eq(growable), expected);
    assert_eq!(exact.eq(&text), expected);
    assert_eq!(growable.eq(&text), expected);
    assert_eq!(<&str as PartialEq<CharStr>>::eq(&text, exact), expected);
    assert_eq!(<&str as PartialEq<CharString>>::eq(&text, growable), expected);

    let owned = text.to_owned();
    assert_eq!(exact.eq(&owned), expected);
    assert_eq!(growable.eq(&owned), expected);
    assert_eq!(owned.eq(exact), expected);
    assert_eq!(owned.eq(growable), expected);
    assert_eq!(growable.eq(&Cow::Borrowed(text)), expected);
    assert_eq!(Cow::Borrowed(text).eq(growable), expected);
}

#[test]
fn every_short_length_alignment_and_mismatch() {
    for offset in 0..size_of::<usize>() {
        for len in 0..=CharStr::INLINE_CAPACITY + 1 {
            // The string ends at its allocation boundary, so Miri also checks the upper load bound.
            let mut storage = vec![b'a'; offset + len].into_boxed_slice();
            let text = std::str::from_utf8(&storage[offset..]).unwrap();
            let exact = CharStr::from(text);
            let growable = CharString::from(text);
            check_equality(&exact, &growable, text);

            for index in 0..len {
                storage[offset + index] = b'b';
                let text = std::str::from_utf8(&storage[offset..]).unwrap();
                check_equality(&exact, &growable, text);
                storage[offset + index] = b'a';
            }

            check_equality(&exact, &growable, &"a".repeat(len + 1));
            if len > 0 {
                check_equality(&exact, &growable, &"a".repeat(len - 1));
            }
        }
    }
}

#[test]
fn truncated_inline_tail_bytes_do_not_affect_equality() {
    for len in 0..=CharStr::INLINE_CAPACITY {
        let mut growable = CharString::from("a".repeat(CharStr::INLINE_CAPACITY).as_str());
        growable.truncate(len);
        let exact = growable.clone().freeze();
        check_equality(&exact, &growable, &"a".repeat(len));

        let mut different_tail = CharString::from(
            ("a".repeat(len) + &"b".repeat(CharStr::INLINE_CAPACITY - len)).as_str(),
        );
        different_tail.truncate(len);
        assert_eq!(growable, different_tail);
        assert_eq!(exact, different_tail.freeze());
    }
}

#[test]
fn short_heap_and_static_storage_match_inline_storage() {
    const TEXT: &str = "abcdefghijklmnopqrstuvwxyz0123456789";
    for len in 0..=CharStr::INLINE_CAPACITY {
        let text = &TEXT[..len];
        let exact = CharStr::new_heap(text);
        let mut growable = CharString::with_capacity(TEXT.len());
        growable.push_str(text);
        check_equality(&exact, &growable, text);
        check_equality(&exact, &growable, &"!".repeat(len));

        let mut static_string = CharString::from_static_str(TEXT);
        static_string.truncate(len);
        assert_eq!(static_string.as_static_str(), Some(text));
        check_equality(&static_string.clone().freeze(), &static_string, text);
        assert_eq!(exact, static_string);
        assert_eq!(growable, static_string);
    }
}

#[test]
fn utf8_and_nul_bytes_match_by_content() {
    for suffix in ["\0", "é", "€", "🦀"] {
        for len in suffix.len()..=CharStr::INLINE_CAPACITY + 1 {
            let text = "x".repeat(len - suffix.len()) + suffix;
            let exact = CharStr::from(text.as_str());
            let growable = CharString::from(text.as_str());
            check_equality(&exact, &growable, &text);
            check_equality(&exact, &growable, &"x".repeat(len));
        }
    }
}
