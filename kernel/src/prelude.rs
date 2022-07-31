pub use crate::{println, kprint as print, kprint, kprintln, prepare_dump};
pub use crate::mem::{PhysicalAddress, VirtualAddress};

pub use core::iter::FromIterator;
pub use core::convert::{TryFrom, TryInto};

// Re-exported core operators
#[doc(no_inline)]
pub use core::marker::{Copy, Send, Sized, Sync, Unpin};
#[doc(no_inline)]
pub use core::ops::{Drop, Fn, FnMut, FnOnce};

// Re-exported functions
#[doc(no_inline)]
pub use core::mem::drop;

// Re-exported types and traits
#[doc(no_inline)]
pub use core::clone::Clone;
#[doc(no_inline)]
pub use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
#[doc(no_inline)]
pub use core::convert::{AsMut, AsRef, From, Into};
#[doc(no_inline)]
pub use core::default::Default;
#[doc(no_inline)]
pub use core::iter::{DoubleEndedIterator, ExactSizeIterator};
#[doc(no_inline)]
pub use core::iter::{Extend, IntoIterator, Iterator};
#[doc(no_inline)]
pub use core::option::Option::{self, None, Some};
#[doc(no_inline)]
pub use core::result::Result::{self, Err, Ok};

// Re-exported built-in macros
#[doc(no_inline)]
pub use core::fmt::Debug;
#[doc(no_inline)]
pub use core::hash::Hash;

#[allow(deprecated)]
#[doc(no_inline)]
pub use core::{
    assert, cfg, column, compile_error, concat, concat_idents, env, file, format_args,
    format_args_nl, include, include_bytes, include_str, line, log_syntax, module_path, option_env,
    stringify, trace_macros,
};


#[doc(no_inline)]
pub use core::concat_bytes;

pub use core::prelude::v1::*;

pub use core::slice;