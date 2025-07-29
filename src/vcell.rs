use core::cell::UnsageCell;

#[repr(transparent)]
pub struct VoilatileCell<T> {
    inner: UnsageCell<T>,
}
