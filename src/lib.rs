//! 
//! A simple crate(library) that provides a way to convert Option to Result.
//! 
//! # Usage
//! 
//! ```
//! use opt2r::OptionToResult;
//! 
//! const CUSTOM_ERROR_CODE_OPTION_IS_NONE: i32 = 1;
//! const CUSTOM_ERROR_STR_OPTION_IS_NONE: &str = "Custom Error: Option is None.";
//! 
//! fn example1() -> opt2r::Result<()> {
//!     let a = make_some().ok_or_()?;
//!     let a = make_some().ok_or(opt2r::opt_is_none!())?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn example2() -> Result<(), i32> {
//!     let a = make_some().ok_or(opt2r::opt_is_none_i32!())?;
//! 
//!     //let b = make_none().ok_or_()?;
//!     let b = make_none().ok_or(opt2r::err!(CUSTOM_ERROR_CODE_OPTION_IS_NONE))?;
//! 
//!     Ok(())
//! }
//! 
//! fn example3() -> Result<(), String> {
//!     let a = make_some().ok_or_()?;
//!     let a = make_some().ok_or(opt2r::opt_is_none!())?;
//!     let a = make_some().ok_or(opt2r::err!(CUSTOM_ERROR_STR_OPTION_IS_NONE))?;
//!     let a = make_some().ok_or(opt2r::Error::new(CUSTOM_ERROR_STR_OPTION_IS_NONE))?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn example4() -> Result<(), Box<dyn opt2r::StdError>> {
//!     let a = make_some().ok_or_()?;
//!     let a = make_some().ok_or(opt2r::opt_is_none!())?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn main() {
//!     if let Err(err) = example1() {
//!         println!("example1 err={}, {}", err, err.error_value());
//!     }
//! 
//!     if let Err(err) = example2() {
//!         println!("example2 err={}", err);
//!     }
//! 
//!     if let Err(err) = example3() {
//!         println!("example3 err={}", err);
//!     }
//! 
//!     //example4().unwrap();
//!     if let Err(err) = example4() {
//!         println!("example4 err={}", err);
//!     }
//! }
//! 
//! fn make_some() -> Option<i32> {
//!     Some(100)
//! }
//! 
//! fn make_none() -> Option<f64> {
//!     None
//! }
//! ```

#[cfg(feature = "std")]
extern crate std;

//#[macro_use]
mod util;

pub const ERROR_CODE_OPTION_IS_NONE: i32 = 1;

pub const STR_OPTION_IS_NONE: &str = "Option is None.";

#[cfg(feature = "std")]
pub use std::error::Error as StdError;

#[cfg(not(feature = "std"))]
pub trait StdError: core::fmt::Debug + core::fmt::Display {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

pub trait OptionToResult<T, V> {
    fn ok_or_(self) -> core::result::Result<T, Error<V>>;
}

#[derive(Debug, Clone)]
pub struct Error<V> {
    error_value: V,
}

pub type Result<T, E = Error<String>> = core::result::Result<T, E>;

impl<V: core::fmt::Debug> core::fmt::Display for Error<V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type1 = core::any::type_name::<Self>();             // => crate::Error
        //let type2 = core::any::type_name_of_val(self);          // => crate::Error
        let type3 = format!("{:?}", self);

        write!(f, "{} :: {}", type1, type3)
    }
}

impl<V: core::fmt::Debug> StdError for Error<V> {
    
}

impl<T> OptionToResult<T, String> for Option<T> {
    fn ok_or_(self) -> Result<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(opt_is_none!()),
        }
    }
}

impl<V> Error<V> {
    pub fn new(error_value: V) -> Self {
        Self {
            error_value,
        }
    }

    pub fn error_value(&self) -> &V {
        &self.error_value
    }
}

macro_rules! impl_from_error_for {
    ($for_type:ty ) => {
        impl<V> From<Error<V>> for $for_type
        where
            V: Into<$for_type>,
        {
            fn from(value: Error<V>) -> Self {
                value.error_value.into()
            }
        }
    };
}

impl_from_error_for!(i32);
impl_from_error_for!(u32);
impl_from_error_for!(String);

#[cfg(not(feature = "std"))]
impl<V: core::fmt::Debug + 'static> From<Error<V>> for Box<dyn StdError> {
    fn from(value: Error<V>) -> Self {
        Box::new(value)
    }
}