
#[cfg(feature = "logging")]
pub use log::*;

#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! trace    ( ($($tt:tt)*) => {{}} );
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! debug    ( ($($tt:tt)*) => {{}} );
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! info     ( ($($tt:tt)*) => {{}} );
#[cfg(not(feature = "logging"))]
#[macro_export]
macro_rules! error    ( ($($tt:tt)*) => {{}} );

#[cfg(not(feature = "logging"))]
#[allow(unused_imports)]
pub use {debug, trace, info, error};
