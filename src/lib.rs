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
//! fn example1(i: i32) -> opt2r::Result<()> {
//!     let a = make_some(i).ok_or(opt2r::opt_is_none!())?;
//!     let a2 = make_some(i).ok_or_()?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn example2(i: i32) -> Result<(), i32> {
//!     let a = make_some(i).ok_or(opt2r::opt_is_none!())?;
//!     let a2 = make_some(i).ok_or(opt2r::opt_is_none_i32!())?;
//! 
//!     let b = make_none().ok_or(opt2r::err_i32!(CUSTOM_ERROR_CODE_OPTION_IS_NONE))?;
//! 
//!     Ok(())
//! }
//! 
//! fn example3(i: i32) -> Result<(), String> {
//!     let a = make_some(i).ok_or(opt2r::opt_is_none!())?;
//!     let a2 = make_some(i).ok_or(opt2r::err_s!(CUSTOM_ERROR_STR_OPTION_IS_NONE))?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn main() {
//!     if let Err(err) = example1(100) {
//!         println!("example1 err={}", err);
//!     }
//! 
//!     if let Err(err) = example2(200) {
//!         println!("example2 err={}", err);
//!     }
//! 
//!     if let Err(err) = example3(300) {
//!         println!("example3 err={}", err);
//!     }
//!     
//!     //example3(300).unwrap();
//! }
//! 
//! fn make_some<T>(v: T) -> Option<T> {
//!     Some(v)
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
pub const STR_FAILED_CONVERT_ERROR_TO_TYPE: &str = "Failed to convert Error to type";

#[cfg(feature = "std")]
pub use std::error::Error as StdError;

#[cfg(not(feature = "std"))]
pub trait StdError: core::fmt::Debug + core::fmt::Display {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

pub type BoxStdError = Box<dyn StdError>;

pub trait OptionToResult<T> {
    fn ok_or_(self) -> core::result::Result<T, Error>;
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    I32Error(i32),
    U32Error(u32),
    StringError(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let type1 = core::any::type_name::<Self>();             // => crate::Error
        //let type2 = core::any::type_name_of_val(self);          // => crate::Error
        let type3 = format!("{:?}", self);

        write!(f, "{} :: {}", type1, type3)
    }
}

impl StdError for Error {
    
}

impl<T> OptionToResult<T> for Option<T> {
    fn ok_or_(self) -> Result<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(opt_is_none!()),
        }
    }
}

macro_rules! impl_error_from {
    ($for_type:ty, $enum_variant:ident) => {
        impl From<Error> for $for_type {
            fn from(value: Error) -> Self {
                match value {
                    Error::$enum_variant(err) => err,
                    _ => panic!("{} {}.", STR_FAILED_CONVERT_ERROR_TO_TYPE, stringify!($for_type)),
                }
            }
        }
    };
}

impl_error_from!(i32, I32Error);
impl_error_from!(u32, U32Error);
impl_error_from!(String, StringError);

#[cfg(not(feature = "std"))]
impl From<Error> for BoxStdError {
    fn from(value: Error) -> Self {
        Box::new(value)
    }
}