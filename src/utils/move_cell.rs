use std::cell::UnsafeCell;
use std::mem;

/// A cell type that can move values into and out of a shared reference.
pub struct MoveCell<T>(UnsafeCell<Option<T>>);

impl<T> MoveCell<T> {
    pub fn new() -> MoveCell<T> {
        MoveCell(UnsafeCell::new(None))
    }

    pub fn put(&self, val: T) -> Option<T> {
        let mut_ref = unsafe { self.as_mut() };
        mem::replace(mut_ref, Some(val))       
    }

    pub fn take(&self) -> Option<T> {
        unsafe { self.as_mut() }.take()
    }

    pub fn clone_inner(&self) -> Option<T> where T: Clone {
        unsafe { self.as_ref() }.clone()
    }

    unsafe fn as_mut(&self) -> &mut Option<T> {
        &mut *self.0.get()
    }

    unsafe fn as_ref(&self) -> &Option<T> {
        & *self.0.get()
    }

    pub fn has_value(&self) -> bool {
        unsafe { self.as_ref() }.is_some()
    }
}

