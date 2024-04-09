#[macro_export]
macro_rules! err {
    ($e:expr) => {
        $crate::Error::new($e)
    };
}

#[macro_export]
macro_rules! opt_is_none {
    () => {
        $crate::err!($crate::STR_OPTION_IS_NONE.to_string())
    };
}

#[macro_export]
macro_rules! opt_is_none_i32 {
    () => {
        $crate::err!($crate::ERROR_CODE_OPTION_IS_NONE)
    };
}

#[macro_export]
macro_rules! opt_is_none_u32 {
    () => {
        $crate::err!($crate::ERROR_CODE_OPTION_IS_NONE as u32)
    };
}