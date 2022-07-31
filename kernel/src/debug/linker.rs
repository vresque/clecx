#[macro_export]
macro_rules! linker_constants {
    ($($name:ident)*) => {
        $(
            #[no_mangle]
            pub static $name: u64 = 0;
        )*
    };
}