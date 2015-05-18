//! A cell type that can move values into and out of a shared reference.
//!
//! Behaves like `RefCell<Option<T>>` but optimized for use-cases where temporary or permanent
//! ownership is required.

use std::cell::UnsafeCell;
use std::mem;

/// A cell type that can move values into and out of a shared reference.
pub struct MoveCell<T>(UnsafeCell<Option<T>>);

impl<T> MoveCell<T> {
    /// Create a new `MoveCell` with no contained value.
    pub fn new() -> MoveCell<T> {
        MoveCell(UnsafeCell::new(None))
    }

    /// Create a new `MoveCell` with the given value.
    pub fn with(val: T) -> MoveCell<T> {
        MoveCell(UnsafeCell::new(None))
    }

    /// Create a new `MoveCell<T>` around the given `Option<T>`.
    pub fn from(opt: Option<T>) -> MoveCell<T> {
        MoveCell(UnsafeCell::new(opt))
    }

    unsafe fn as_mut(&self) -> &mut Option<T> {
        &mut *self.0.get()
    }

    unsafe fn as_ref(&self) -> &Option<T> {
        & *self.0.get()
    }

    /// Place a value into this `MoveCell`, returning the previous value, if present.
    pub fn put(&self, val: T) -> Option<T> {
        mem::replace(unsafe { self.as_mut() }, Some(val))       
    }

    /// Take the value out of this `MoveCell`, leaving nothing in its place.
    pub fn take(&self) -> Option<T> {
        unsafe { self.as_mut().take() }
    }

    /// Take the value out of this `MoveCell`, leaving a clone in its place. 
    pub fn clone_inner(&self) -> Option<T> where T: Clone {
        let inner = self.take();
        inner.clone().map(|inner| self.put(inner));
        inner
    }

    /// Check if this `MoveCell` contains a value or not.
    pub fn has_value(&self) -> bool {
        unsafe { self.as_ref().is_some() }
    }
}

impl<T> Default for MoveCell<T> {
    fn default() -> Self {
        MoveCell::new()
    }
}

