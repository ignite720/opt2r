//! 
//! A simple crate(library) that provides a way to convert Option to Result.
//! 
//! # Usage
//! 
//! ```
//! use opt2r::OptionToResult;
//! use opt2r::Result;
//! 
//! fn example1(i: i32) -> Result<()> {
//!     let a = make_some(i).ok_or(opt2r::opt_is_none!())?;
//!     let a2 = make_some(i).ok_or_()?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn example2(i: i32) -> Result<(), String> {
//!     let a = make_some(i).ok_or(opt2r::opt_is_none!())?;
//!     let a2 = make_some(i).ok_or_()?;
//! 
//!     let b = make_none().ok_or_()?;
//! 
//!     Ok(())
//! }
//! 
//! fn main() {
//!     if let Err(err) = example1(100) {
//!         println!("{}", err);
//!     }
//!     if let Err(err) = example2(200) {
//!         println!("{}", err);
//!     }
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

#[macro_use]
mod util;

pub const ERR_OPTION_IS_NONE: &str = "Option is None";
pub const ERR_FAILED_CONVERT_ERROR_TO_TYPE: &str = "Failed to convert Error to type";

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
        match self {
            Error::I32Error(err) => write!(f, "{}", err),
            Error::U32Error(err) => write!(f, "{}", err),
            Error::StringError(err) => write!(f, "{}", err),
        }
    }
}

impl StdError for Error {
    
}

impl<T> OptionToResult<T> for Option<T> {
    fn ok_or_(self) -> Result<T> {
        match self {
            Some(v) => Ok(v),
            None => Err(Error::StringError(ERR_OPTION_IS_NONE.into())),
        }
    }
}

macro_rules! impl_error_from {
    ($for_type:ty, $enum_variant:ident) => {
        impl From<Error> for $for_type {
            fn from(value: Error) -> Self {
                match value {
                    Error::$enum_variant(err) => err,
                    _ => panic!("{} {}.", ERR_FAILED_CONVERT_ERROR_TO_TYPE, stringify!($for_type)),
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