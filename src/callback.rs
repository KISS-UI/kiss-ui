//! Traits for notifying client code when the state of a KISS-UI widget is updated.

use widget_prelude::*;

use iup_sys::Ihandle;

use std::cell::RefCell;
use std::collections::HashMap;

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
    pub fn close(&mut self) {
        *self = CallbackStatus::Close;
    }

    #[doc(hidden)]
    pub fn to_cb_return(self) -> ::libc::c_int {
        use self::CallbackStatus::*;

        match self {
            Close => ::iup_sys::IUP_CLOSE,
            Default => ::iup_sys::IUP_DEFAULT,
            // _ => unimplemented!(),
        }
    }
}

impl From<()> for CallbackStatus {
    fn from(_: ()) -> CallbackStatus {
        CallbackStatus::Default
    }
}

pub trait Callback<Args>: 'static {
    fn on_callback(&mut self, args: Args) -> CallbackStatus; 
}

impl<Args, Out: Into<CallbackStatus>, F: 'static> Callback<Args> for F where F: FnMut(Args) -> Out {
    /// Because of the `impl From<()> for CallbackStatus`, closures that return `()` can be
    /// accepted by this impl.
    fn on_callback(&mut self, args: Args) -> CallbackStatus {
        self(args).into()
    }
}

#[doc(hidden)]
pub type CallbackMap<T> = RefCell<HashMap<*mut Ihandle, *mut Callback<T>>>;

#[doc(hidden)]
pub fn new_callback_map<T>() -> CallbackMap<T> {
    RefCell::new(HashMap::new())    
}

macro_rules! callback_impl {
    ($cb_attr:expr, $self_val:expr, $callback:expr, $self_ty:ident) => (
        { 
            thread_local!(
                static CALLBACKS: ::callback::CallbackMap<$self_ty> = 
                    ::callback::new_callback_map()
            );

            fn callback_glue(element: *mut ::iup_sys::Ihandle)
            -> ::libc::c_int {
                use ::callback::CallbackStatus;

                let widget = unsafe { $self_ty::from_ptr(element) };

                CALLBACKS.with(|callbacks| 
                    callbacks.borrow()
                        .get(&widget.ptr()).cloned()
                )
                .map(|callback| unsafe {
                    (*callback).on_callback(widget)
                })                    
                .unwrap_or(CallbackStatus::Default).to_cb_return()
            }

            extern fn ffi_callback_glue(element: *mut ::iup_sys::Ihandle) 
            -> ::libc::c_int {
                let _guard = ::PanicGuard;
                callback_glue(element)
            }
            
            let mut callback = Box::new($callback);

            CALLBACKS.with(|callbacks| 
                callbacks.borrow_mut().insert($self_val.ptr(), &mut *callback)
            );
            
            $self_val.set_callback($cb_attr, ffi_callback_glue);
            ::KISSContext::store_for_drop(callback);
        }
    )
}

/// A trait describing a widget that can be clicked, and can notify client code when this occurs.
pub trait OnClick: Widget {
    fn set_onclick<Cb>(self, on_click: Cb) -> Self where Cb: Callback<Self>;
}

macro_rules! impl_onclick {
    ($self_ty:ident) => (
        impl $crate::callback::OnClick for $self_ty {
            fn set_onclick<Cb>(self, on_click: Cb) -> Self where Cb: ::callback::Callback<Self> {
                callback_impl! { $crate::attrs::ACTION, self, on_click, $self_ty }
                self
            }
        }
    )
}

/// A trait describing a widget which has a value that can be changed by the user, and can notify
/// client code when this occurs.
pub trait OnValueChange: Widget {
    fn set_on_value_changed<Cb>(self, on_value_chaged: Cb) -> Self where Cb: Callback<Self>;
}

macro_rules! impl_on_value_change {
    ($self_ty:ident) => (
        impl $crate::callback::OnValueChange for $self_ty {
            fn set_on_value_changed<Cb>(self, on_value_changed: Cb) -> Self where Cb: ::callback::Callback<Self> {
                callback_impl! { $crate::attrs::VALUE_CHANGED_CB, self, on_value_changed, $self_ty }
                self
            }
        }
    )
}

/// A trait describing a widget that can be shown, and can notify client code when this occurs.
pub trait OnShow: Widget {
    fn set_on_show<Cb>(self, on_show: Cb) -> Self where Cb: Callback<Self>;
}

macro_rules! impl_on_show {
    ($self_ty:ident) => (
        impl ::callback::OnShow for $self_ty {
            fn set_on_show<Cb>(self, on_show: Cb) -> Self where Cb: ::callback::Callback<Self> {
                callback_impl! { ::attrs::MAP_CB, self, on_show, $self_ty }
                self
            }
        }
    )
}
