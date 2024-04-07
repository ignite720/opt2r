#[macro_export]
macro_rules! opt_is_none {
    () => {
        {
            let err = opt2r::Error::StringError(opt2r::ERR_FAILED_CONVERT_ERROR_TO_TYPE.into());
            err
        }
    };
}