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

macro_rules! impl_callback {
    ($cb_ty:ty, $cb_fn:ident($cb_action:expr) for $self_ty:ty ) => (
        impl $cb_ty for $self_ty {
            fn $cb_fn<F>(mut self, callback: F) -> $self_ty 
                where F: FnMut($self_ty) + 'static 
            { 
                thread_local!(
                    static CALLBACKS: $crate::callback::CallbackMap<$self_ty> = 
                        ::std::cell::RefCell::new(::std::collections::HashMap::new())
                );

                extern fn extern_callback(element: *mut $crate::iup_sys::Ihandle) 
                    -> $crate::iup_sys::CallbackReturn 
                {
                    let cb_status = &::std::cell::Cell::new($crate::callback::CallbackStatus::Default);
                    if let Ok(_self) = $crate::BaseWidget::from_ptr(element)
                        .downcast::<$self_ty>() 
                    {
                        $crate::callback::CB_RETURN.set(cb_status, ||            
                            CALLBACKS.with(|callbacks| 
                                callbacks.borrow_mut()
                                    .get_mut(&_self.as_ptr())
                                    .map(|cb| cb(_self))
                            )
                        );
                    }

                    cb_status.get().to_cb_return() 
                }

                CALLBACKS.with(|callbacks| 
                    callbacks.borrow_mut()
                        .insert(self.as_ptr(), Box::new(callback))
                );
                self.set_callback($cb_action, extern_callback);

                self
                
            }
        }
    )
}

pub trait OnClick: DerefMut<Target=BaseWidget> + Sized {
    fn set_onclick<F>(self, on_click: F) -> Self where F: FnMut(Self) + 'static;
}

macro_rules! impl_onclick {
    ($self_ty:ty) => (impl_callback! { $crate::callback::OnClick, set_onclick($crate::attrs::ACTION) for $self_ty })
}

