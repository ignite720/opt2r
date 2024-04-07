#[macro_export]
macro_rules! err_i32 {
    ($e:expr) => {
        $crate::Error::I32Error($e)
    };
}

#[macro_export]
macro_rules! err_u32 {
    ($e:expr) => {
        $crate::Error::U32Error($e)
    };
}

#[macro_export]
macro_rules! err_s {
    ($e:expr) => {
        $crate::Error::StringError($e.into())
    };
}

#[macro_export]
macro_rules! opt_is_none {
    () => {
        $crate::err_s!($crate::STR_OPTION_IS_NONE)
    };
}

#[macro_export]
macro_rules! opt_is_none_i32 {
    () => {
        $crate::err_i32!($crate::ERROR_CODE_OPTION_IS_NONE)
    };
}

#[macro_export]
macro_rules! opt_is_none_u32 {
    () => {
        $crate::err_u32!($crate::ERROR_CODE_OPTION_IS_NONE as u32)
    };
}