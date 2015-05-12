//! Traits for notifying client code when the state of a KISS-UI widget is updated.

use super::BaseWidget;

use iup_sys::{Ihandle, CallbackReturn};

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::DerefMut;

#[doc(hidden)]
scoped_thread_local!(pub static CB_RETURN: Cell<CallbackStatus>);

/// Set this within a callback to tell the framework if it should close or not.
///
/// If `Callback::close.set()` is called within a callback, then when the callback returns,
/// the dialog containing the widget on which the callback was invoked will be closed.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CallbackStatus {
    //Ignore,
    /// The default `CallbackStatus`, does nothing when set.
    Default,
    /// If this is set within a callback, then when the callback returns the dialog containing the
    /// widget on which the callback was invoked will be closed.
    Close,
    //Continue,
}

impl CallbackStatus {
    /// Set this `CallbackStatus` within a callback.
    ///
    /// ##Panics
    /// If this is called outside of a KISS-UI callback.
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

pub trait Callback<Args>: 'static {
    fn on_callback(&mut self, args: Args); 
}

impl<Args, F: FnMut<Args, Output=()> + 'static> Callback<Args> for F {
    fn on_callback(&mut self, args: Args) {
        self.call_mut(args);
    }
}

#[doc(hidden)]
pub type CallbackMap<T> = RefCell<HashMap<*mut Ihandle, Box<Callback<T>>>>;

macro_rules! callback_impl {
    ($cb_attr:expr, $base:expr, $callback:expr, $self_ty:ty) => (
        { 
            thread_local!(
                static CALLBACKS: $crate::callback::CallbackMap<($self_ty,)> = 
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
                                .map(|cb| cb.on_callback((_self,)))
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

/// A trait describing a widget that can be clicked, and can notify client code when this occurs.
pub trait OnClick: DerefMut<Target=BaseWidget> + Sized {
    fn set_onclick<Cb>(self, on_click: Cb) -> Self where Cb: Callback<(Self,)>;
}

macro_rules! impl_onclick {
    ($self_ty:ty) => (
        impl $crate::callback::OnClick for $self_ty {
            fn set_onclick<Cb>(mut self, on_click: Cb) -> Self where Cb: ::callback::Callback<(Self,)> {
                callback_impl! { $crate::attrs::ACTION, self, on_click, $self_ty }
                self
            }
        }
    )
}

/// A trait describing a widget which has a value that can be changed by the user, and can notify
/// client code when this occurs.
pub trait OnValueChange: DerefMut<Target=BaseWidget> + Sized {
    fn set_on_value_changed<Cb>(self, on_value_chaged: Cb) -> Self where Cb: Callback<(Self,)>;
}

macro_rules! impl_on_value_change {
    ($self_ty:ty) => (
        impl $crate::callback::OnValueChange for $self_ty {
            fn set_on_value_changed<Cb>(mut self, on_value_changed: Cb) -> Self where Cb: ::callback::Callback<(Self,)> {
                callback_impl! { $crate::attrs::VALUE_CHANGED_CB, self, on_value_changed, $self_ty }
                self
            }
        }
    )
}

/// A trait describing a widget that can be shown, and can notify client code when this occurs.
pub trait OnShow: DerefMut<Target=BaseWidget> + Sized {
    fn set_on_show<Cb>(self, on_show: Cb) -> Self where Cb: Callback<(Self,)>;
}

macro_rules! impl_on_show {
    ($self_ty:ty) => (
        impl ::callback::OnShow for $self_ty {
            fn set_on_show<Cb>(mut self, on_show: Cb) -> Self where Cb: ::callback::Callback<(Self,)> {
                callback_impl! { ::attrs::MAP_CB, self, on_show, $self_ty }
                self
            }
        }
    )
}
