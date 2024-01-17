mod bindgen;

#[cfg(feature = "injector")]
extern crate link_cplusplus;

#[cfg(feature = "injectee")]
pub mod postjectee;

#[cfg(feature = "injector")]
pub mod postjector;
