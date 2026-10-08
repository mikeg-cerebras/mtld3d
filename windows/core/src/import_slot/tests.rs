use super::replace;

#[test]
fn replace_and_restore_work_at_every_pointer_alignment_without_touching_neighbors() {
    let original_value = 1u8;
    let replacement_value = 2u8;
    let original = (&raw const original_value).cast::<()>();
    let replacement = (&raw const replacement_value).cast::<()>();
    let width = size_of::<usize>();
    for offset in 0..width {
        let mut storage = [0usize; 4];
        let aligned = storage.as_mut_ptr().cast::<*const ()>();
        let base = aligned.cast::<u8>();
        let start = width + offset;
        // SAFETY: start leaves a complete pointer plus guards inside this owned storage.
        let slot = unsafe { aligned.byte_add(start) };
        // SAFETY: the slot is writable and has no competing accesses; alignment is not required.
        unsafe { slot.write_unaligned(original) };
        // SAFETY: the initialized storage is still live; no reference survives this copy.
        let before = unsafe { core::slice::from_raw_parts(base, size_of_val(&storage)) }.to_vec();

        // SAFETY: this is an initialized, exclusively owned pointer-sized slot.
        assert_eq!(unsafe { replace(slot, replacement) }, original);
        // SAFETY: the operation left a complete initialized pointer in the slot.
        assert_eq!(unsafe { slot.read_unaligned() }, replacement);
        // SAFETY: the initialized storage is still live and not mutated during this copy.
        let after = unsafe { core::slice::from_raw_parts(base, size_of_val(&storage)) }.to_vec();
        assert_eq!(&after[..start], &before[..start]);
        assert_eq!(&after[start + width..], &before[start + width..]);

        // SAFETY: the same slot remains initialized and exclusively owned.
        assert_eq!(unsafe { replace(slot, original) }, replacement);
        // SAFETY: the initialized storage is still live and no further mutation follows.
        let restored = unsafe { core::slice::from_raw_parts(base, size_of_val(&storage)) };
        assert_eq!(restored, before);
    }
}
