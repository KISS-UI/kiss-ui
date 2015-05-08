use super::BaseWidget;

use iup_sys::{Ihandle, CallbackReturn};

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::DerefMut;

#[doc(hidden)]
scoped_thread_local!(pub static CB_RETURN: Cell<CallbackStatus>);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CallbackStatus {
    //Ignore,
    Default,
    Close,
    //Continue,
}

impl CallbackStatus {
    pub fn set(self) {
        assert!(CB_RETURN.is_set(), "CallbackStatus cannot be set outside of a callback!");
        CB_RETURN.with(|cb_ret| cb_ret.set(self));         
    }

    #[doc(hidden)]
    pub fn to_cb_return(self) -> CallbackReturn {
        use self::CallbackStatus::*;

        match self {
            Close => CallbackReturn::Close,
            Default => CallbackReturn::Default,
            // _ => unimplemented!(),
        }
    }
}

#[doc(hidden)]
pub type CallbackMap<T> = RefCell<HashMap<*mut Ihandle, Box<FnMut(T) + 'static>>>;

macro_rules! callback_impl {
    ($cb_attr:expr, $base:expr, $callback:expr, $self_ty:ty) => (
        { 
            thread_local!(
                static CALLBACKS: $crate::callback::CallbackMap<$self_ty> = 
                    ::std::cell::RefCell::new(::std::collections::HashMap::new())
            );

            extern fn extern_callback(element: *mut $crate::iup_sys::Ihandle) 
                -> $crate::iup_sys::CallbackReturn 
            {
                let cb_status = &::std::cell::Cell::new($crate::callback::CallbackStatus::Default);
                if let Ok(_self) = unsafe { $crate::BaseWidget::from_ptr(element) }
                    .try_downcast::<$self_ty>() 
                {
                    $crate::callback::CB_RETURN.set(cb_status, ||            
                        CALLBACKS.with(|callbacks| 
                            callbacks.borrow_mut()
                                .get_mut(&_self.ptr())
                                .map(|cb| cb(_self))
                        )
                    );
                }

                cb_status.get().to_cb_return() 
            }

            CALLBACKS.with(|callbacks| 
                callbacks.borrow_mut()
                    .insert($base.ptr(), Box::new($callback))
            );
            $base.set_callback($cb_attr, extern_callback);                
        }
    )
}

pub trait OnClick: DerefMut<Target=BaseWidget> + Sized {
    fn set_onclick<F>(self, on_click: F) -> Self where F: FnMut(Self) + 'static;
}

macro_rules! impl_onclick {
    ($self_ty:ty) => (
        impl $crate::callback::OnClick for $self_ty {
            fn set_onclick<F>(mut self, on_click: F) -> Self where F: FnMut(Self) + 'static {
                callback_impl! { $crate::attrs::ACTION, self, on_click, $self_ty }
                self
            }
        }
    )
}

pub trait OnValueChange: DerefMut<Target=BaseWidget> + Sized {
    fn set_on_value_changed<F>(self, on_value_chaged: F) -> Self where F: FnMut(Self) + 'static;
}

macro_rules! impl_on_value_change {
    ($self_ty:ty) => (
        impl $crate::callback::OnValueChange for $self_ty {
            fn set_on_value_changed<F>(mut self, on_value_changed: F) -> Self where F: FnMut(Self) + 'static {
                callback_impl! { $crate::attrs::VALUE_CHANGED_CB, self, on_value_changed, $self_ty }
                self
            }
        }
    )
}

