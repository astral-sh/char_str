use super::*;

#[cfg(target_pointer_width = "64")]
#[repr(C, align(8))]
pub(super) struct InlineBuffer([u8; MAX_INLINE_SIZE]);

#[cfg(target_pointer_width = "32")]
#[repr(C, align(4))]
pub(super) struct InlineBuffer([u8; MAX_INLINE_SIZE]);

const _: () = {
    assert!(size_of::<InlineBuffer>() == MAX_INLINE_SIZE);
    assert!(align_of::<InlineBuffer>() == align_of::<usize>());
};

impl InlineBuffer {
    /// # Safety
    /// `text` must have a length less than or equal to `MAX_INLINE_SIZE`.
    pub(super) const unsafe fn new(text: &str) -> Self {
        debug_assert!(text.len() <= MAX_INLINE_SIZE);

        let len = text.len();
        let mut buffer = [0u8; MAX_INLINE_SIZE];
        buffer[MAX_INLINE_SIZE - 1] = len as u8 | LastByte::MASK_1100_0000;

        // SAFETY:
        // - src (`text`) and dst (`ptr`) is valid for `len` bytes.
        // - Both src and dst is aligned for u8.
        // - src and dst don't overlap because we created dst.
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), buffer.as_mut_ptr(), len);
        }

        Self(buffer)
    }

    pub(super) fn from_joined_slices<T: AsRef<str>>(
        slices: &[T],
        separator: &str,
        text_len: usize,
    ) -> Result<Self, ReserveError> {
        debug_assert!(text_len <= MAX_INLINE_SIZE);

        let mut buffer = Self::empty();
        let mut offset = 0;

        for (index, text) in slices.iter().enumerate() {
            if index > 0 {
                buffer.copy_part(separator, &mut offset, text_len)?;
            }
            buffer.copy_part(text.as_ref(), &mut offset, text_len)?;
        }

        if offset != text_len {
            return Err(ReserveError);
        }

        // SAFETY: Every copied part was valid UTF-8, and the checked final offset proves that
        // exactly `text_len <= MAX_INLINE_SIZE` bytes were initialized.
        unsafe { buffer.set_len(text_len) };
        Ok(buffer)
    }

    fn copy_part(
        &mut self,
        text: &str,
        offset: &mut usize,
        text_len: usize,
    ) -> Result<(), ReserveError> {
        let end = offset.checked_add(text.len()).ok_or(ReserveError)?;
        if end > text_len {
            return Err(ReserveError);
        }

        // SAFETY: The bounds check above proves the destination is valid for `text.len()` bytes.
        // The source is a valid string slice and cannot overlap this stack buffer.
        unsafe {
            ptr::copy_nonoverlapping(text.as_ptr(), self.0.as_mut_ptr().add(*offset), text.len());
        }
        *offset = end;
        Ok(())
    }

    pub(super) const fn empty() -> Self {
        let mut buffer = [0; MAX_INLINE_SIZE];
        buffer[MAX_INLINE_SIZE - 1] = LastByte::Length00 as u8;
        Self(buffer)
    }

    /// # Safety
    /// - `len` bytes in the buffer must be valid UTF-8.
    /// - `len` must be less than or equal to `MAX_INLINE_SIZE`.
    pub(super) unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= MAX_INLINE_SIZE);

        if len < MAX_INLINE_SIZE {
            self.0[MAX_INLINE_SIZE - 1] = len as u8 | LastByte::MASK_1100_0000;
        }
    }

    /// Clears the unused bytes while preserving the inline length marker.
    pub(super) fn make_canonical(&mut self, len: usize) {
        debug_assert!(len <= MAX_INLINE_SIZE);

        // Clear in machine-word-sized chunks so the bounded operation stays inline instead of
        // becoming an out-of-line memset for at most `MAX_INLINE_SIZE - 1` bytes.
        let word_size = size_of::<usize>();
        let (head, tail) = self.0.split_at_mut(word_size);
        if len < word_size {
            let mask = (1usize << (len * 8)).wrapping_sub(1);
            let word = usize::from_ne_bytes(head.try_into().unwrap());
            head.copy_from_slice(&(word & mask.to_le()).to_ne_bytes());
            tail[..word_size - 1].fill(0);
        } else if len < MAX_INLINE_SIZE - 1 {
            let mask = (1usize << ((len - word_size) * 8)).wrapping_sub(1)
                | (usize::MAX << ((word_size - 1) * 8));
            let word = usize::from_ne_bytes(tail.try_into().unwrap());
            tail.copy_from_slice(&(word & mask.to_le()).to_ne_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonicalize_clears_unused_bytes_and_preserves_marker() {
        let full = "a".repeat(MAX_INLINE_SIZE);

        for len in 0..=MAX_INLINE_SIZE {
            // SAFETY: `full` has exactly `MAX_INLINE_SIZE` bytes.
            let mut buffer = unsafe { InlineBuffer::new(&full) };
            // SAFETY: ASCII prefixes are valid UTF-8 and `len <= MAX_INLINE_SIZE`.
            unsafe { buffer.set_len(len) };
            buffer.make_canonical(len);

            assert_eq!(&buffer.0[..len], &full.as_bytes()[..len]);
            if len < MAX_INLINE_SIZE {
                assert!(buffer.0[len..MAX_INLINE_SIZE - 1].iter().all(|byte| *byte == 0));
                assert_eq!(buffer.0[MAX_INLINE_SIZE - 1], len as u8 | LastByte::MASK_1100_0000);
            }
        }
    }
}
