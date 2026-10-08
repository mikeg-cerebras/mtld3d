//! Pointer access for packed import address tables.

/// Replace a pointer-sized import slot without requiring pointer alignment.
///
/// # Safety
///
/// `slot` must be non-null, readable and writable for one initialized pointer.
/// The caller must exclude concurrent access to these bytes for this operation.
pub const unsafe fn replace(slot: *mut *const (), value: *const ()) -> *const () {
    // SAFETY: the caller supplies a readable, initialized pointer-sized slot.
    let previous = unsafe { slot.read_unaligned() };
    // SAFETY: the caller supplies exclusive writable access to those same bytes.
    unsafe { slot.write_unaligned(value) };
    previous
}

#[cfg(test)]
mod tests;
