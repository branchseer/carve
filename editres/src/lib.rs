#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

#[cfg(feature = "injector")]
pub mod injector;

#[cfg(feature = "injectee")]
#[doc(hidden)]
pub mod injectee;

#[doc(hidden)]
pub mod names;
