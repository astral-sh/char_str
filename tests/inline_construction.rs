use char_str::{CharStr, CharString};

static CONST_STRINGS: [CharStr; 6] = [
    CharStr::from_static_str(""),
    CharStr::from_static_str("a"),
    CharStr::from_static_str("abc"),
    CharStr::from_static_str("abcdefgh"),
    CharStr::from_static_str("abcdefghijklmno"),
    CharStr::from_static_str("abcdefghijklmnop"),
];

#[test]
fn const_construction_matches_runtime_construction() {
    for (value, text) in CONST_STRINGS.iter().zip([
        "",
        "a",
        "abc",
        "abcdefgh",
        "abcdefghijklmno",
        "abcdefghijklmnop",
    ]) {
        assert_eq!(value, &CharStr::from(text));
        assert_eq!(value.as_str(), text);
    }
}

#[test]
fn constructs_every_inline_length_and_alignment() {
    for offset in 0..8 {
        for len in 0..=CharStr::INLINE_CAPACITY + 1 {
            let storage = (0..offset + len).map(|i| b'a' + i as u8).collect::<Vec<_>>();
            let text = std::str::from_utf8(&storage[offset..]).unwrap();
            let frozen = CharStr::from(text);
            let string = CharString::from(text);
            assert_eq!(frozen.as_bytes(), text.as_bytes());
            assert_eq!(string.as_bytes(), text.as_bytes());
            assert_eq!(frozen.is_heap_allocated(), len > CharStr::INLINE_CAPACITY);
            assert_eq!(string.is_heap_allocated(), len > CharStr::INLINE_CAPACITY);
            assert_eq!(
                CharStr::new_inline(text),
                (len <= CharStr::INLINE_CAPACITY).then_some(frozen)
            );
        }
    }
}

#[test]
fn full_inline_utf8_uses_the_final_content_byte() {
    for suffix in ["é", "€", "🦀"] {
        for len in suffix.len()..=CharStr::INLINE_CAPACITY {
            let text = "x".repeat(len - suffix.len()) + suffix;
            assert_eq!(CharStr::new_inline(&text).unwrap(), text);
            assert_eq!(CharString::from(text.as_str()), text);
        }
    }
}

#[test]
fn appends_and_joins_every_inline_length_and_alignment() {
    for source_offset in 0..8 {
        for len in 0..=CharStr::INLINE_CAPACITY + 1 {
            let storage = (0..source_offset + len).map(|i| b'a' + i as u8).collect::<Vec<_>>();
            let text = std::str::from_utf8(&storage[source_offset..]).unwrap();
            for split in 0..=len {
                let (prefix, suffix) = text.split_at(split);
                let mut appended = CharString::from(prefix);
                appended.push_str(suffix);
                assert_eq!(appended.as_str(), text);
                assert_eq!(appended.is_heap_allocated(), len > CharStr::INLINE_CAPACITY);
                assert_eq!(CharStr::concat(&[prefix, suffix]).as_str(), text);
                assert_eq!(CharStr::join(&[prefix, ""], suffix).as_str(), text);
            }
        }
    }
}

#[test]
fn inline_append_and_join_preserve_utf8_and_truncated_contents() {
    for suffix in ["é", "€", "🦀"] {
        for len in suffix.len()..=CharStr::INLINE_CAPACITY {
            let prefix = "x".repeat(len - suffix.len());
            let text = prefix.clone() + suffix;
            let mut appended = CharString::from("x".repeat(CharStr::INLINE_CAPACITY));
            appended.truncate(prefix.len());
            appended.push_str(suffix);
            assert_eq!(appended, text);
            assert_eq!(CharStr::concat(&[prefix.as_str(), suffix]), text);
            assert_eq!(CharStr::join(&[prefix.as_str(), ""], suffix), text);
        }
    }
}
