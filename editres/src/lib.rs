#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "injector")]
pub mod injector;

#[cfg(feature = "injectee")]
#[doc(hidden)]
pub mod injectee;

#[doc(hidden)]
pub mod names;
