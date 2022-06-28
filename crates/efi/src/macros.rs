#[macro_export]
macro_rules! wrap_proto {
    (
        $n:ident wraps $ty:ty; $guid:ident
    ) => {
        pub struct $n(pub &'static mut $ty);

        impl crate::proto::Protocol<$ty> for $n {
            fn guid() -> crate::types::Guid {
                crate::raw::guid::$guid
            }

            fn new(inner: &'static mut $ty) -> Self {
                Self(inner)
            }
        }
    };
}

#[macro_export]
macro_rules! addr {
    ($a:tt) => {
        ($a as *const _)
    };
    (mut $a:tt) => {
        ($a as *mut _)
    };
}

#[macro_export]
macro_rules! as_slice {
    ($val:ident) => {
        unsafe {
            core::slice::from_raw_parts(
                &$val as *const _ as *const u8,
                core::mem::size_of_val(&$val),
            )
        }
    };
    (mut $val:ident) => {
        unsafe {
            core::slice::from_raw_parts_mut(
                &mut $val as *mut _ as *mut u8,
                core::mem::size_of_val(&$val),
            )
        }
    };
}
