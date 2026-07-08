use std::{
    alloc::{GlobalAlloc, Layout, System},
    ptr,
    sync::atomic::{AtomicBool, Ordering},
};

use lean_string::{LeanString, ReserveError};

struct FailNextAllocation;

static FAIL_NEXT_ALLOCATION: AtomicBool = AtomicBool::new(false);

// SAFETY: Allocations are delegated to `System`, except for the single allocation explicitly
// rejected by the test.
unsafe impl GlobalAlloc for FailNextAllocation {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if FAIL_NEXT_ALLOCATION.swap(false, Ordering::SeqCst) {
            ptr::null_mut()
        } else {
            // SAFETY: The caller provides a valid layout.
            unsafe { System.alloc(layout) }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: `ptr` was allocated by `System` with this layout.
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: FailNextAllocation = FailNextAllocation;

#[test]
fn try_freeze_reports_allocation_failure() {
    let mut string = LeanString::with_capacity(128);
    string.push_str("a string longer than the inline limit");

    FAIL_NEXT_ALLOCATION.store(true, Ordering::SeqCst);
    let result = string.try_freeze();

    assert_eq!(result, Err(ReserveError));
}
